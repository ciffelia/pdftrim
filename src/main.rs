use std::{
    io::{BufRead, BufReader},
    process::{Command, Stdio},
};

use clap::Parser;
use log::debug;
use lopdf::Document;
use regex::Regex;

fn main() {
    env_logger::init();

    let args = Args::parse();

    let gs_cmd = find_ghostscript();
    debug!("Using Ghostscript command: {}", gs_cmd);

    let input_path = if std::fs::metadata(&args.input).is_ok() {
        args.input
    } else {
        let with_extension = args.input.clone() + ".pdf";
        if std::fs::metadata(&with_extension).is_ok() {
            with_extension
        } else {
            panic!("Input file `{}' not found", args.input);
        }
    };
    debug!("Input file: {}", input_path);

    let output_path = args.output.clone().unwrap_or_else(|| {
        input_path
            .strip_suffix(".pdf")
            .unwrap_or(&input_path)
            .to_string()
            + "-crop.pdf"
    });
    debug!("Output file: {}", output_path);

    let bboxes = compute_bounding_boxes(&input_path, gs_cmd);
    debug!("Computed bounding boxes for all {} pages", bboxes.len());

    crop_pdf(&input_path, &output_path, &bboxes);

    println!(
        "==> {} page{} written on `{}'.",
        bboxes.len(),
        if bboxes.len() == 1 { "" } else { "s" },
        &output_path
    );
}

#[derive(clap::Parser)]
struct Args {
    #[arg(value_name = "input[.pdf]")]
    input: String,

    #[arg(value_name = "output file")]
    output: Option<String>,
}

fn find_ghostscript() -> &'static str {
    let candidates: &[&str] = if cfg!(windows) {
        &["gswin64c", "gswin32c", "gs"]
    } else {
        &["gs", "gsc", "gswin64c", "gswin32c"]
    };

    for candidate in candidates {
        let result = Command::new(candidate)
            .arg("-h")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status();
        if let Ok(status) = result {
            if status.success() {
                return candidate;
            }
        }
    }

    "gs"
}

fn compute_bounding_boxes(pdf_file: &str, gs_cmd: &str) -> Vec<[f64; 4]> {
    let mut child = Command::new(gs_cmd)
        .args([
            "-dSAFER",
            "-sDEVICE=bbox",
            "-dBATCH",
            "-dNOPAUSE",
            "-c",
            "save",
            "pop",
            "-f",
            pdf_file,
        ])
        .stderr(Stdio::piped())
        .stdout(Stdio::null())
        .spawn()
        .expect("Failed to execute Ghostscript");

    let stderr = BufReader::new(child.stderr.take().unwrap());

    let mut bboxes = Vec::new();
    let mut hires_bboxes = Vec::new();

    let re =
        Regex::new(r"^%%(HiRes)?BoundingBox:\s*(-?[\.\d]+) (-?[\.\d]+) (-?[\.\d]+) (-?[\.\d]+)")
            .unwrap();

    for line in stderr.lines() {
        let line = line.expect("Failed to read output from Ghostscript");
        debug!("{}", line);

        if let Some(caps) = re.captures(&line) {
            let x_min = caps.get(2).unwrap().as_str().parse::<f64>().unwrap();
            let y_min = caps.get(3).unwrap().as_str().parse::<f64>().unwrap();
            let x_max = caps.get(4).unwrap().as_str().parse::<f64>().unwrap();
            let y_max = caps.get(5).unwrap().as_str().parse::<f64>().unwrap();
            if caps.get(1).is_some() {
                hires_bboxes.push([x_min, y_min, x_max, y_max]);
            } else {
                bboxes.push([x_min, y_min, x_max, y_max]);
            }
        }
    }

    let status = child.wait().unwrap();
    if !status.success() {
        panic!("Failed to execute Ghostscript: {}", status);
    }

    if bboxes.is_empty() {
        panic!("No bounding boxes were found in the PDF");
    }
    if hires_bboxes.is_empty() {
        bboxes
    } else {
        assert_eq!(
            bboxes.len(),
            hires_bboxes.len(),
            "Mismatch between the number of BoundingBox and HiResBoundingBox"
        );
        hires_bboxes
    }
}

fn crop_pdf(input_path: &str, output_path: &str, crop_boxes: &[[f64; 4]]) {
    let mut doc = Document::load(input_path).expect("Failed to load the PDF");
    debug!("Loaded input file");

    let page_ids = doc.page_iter().collect::<Vec<_>>();
    assert_eq!(
        page_ids.len(),
        crop_boxes.len(),
        "Page count mismatch between Ghostscript and the PDF parser"
    );

    for (id, crop_box) in page_ids.iter().zip(crop_boxes.iter()) {
        let page = doc
            .objects
            .get_mut(id)
            .unwrap()
            .as_dict_mut()
            .expect("Failed to parse PDF: page is not a dictionary");

        page.set(
            "CropBox",
            crop_box.iter().map(|&x| x.into()).collect::<Vec<_>>(),
        );
    }
    debug!("Updated CropBox for all pages");

    doc.save(output_path)
        .expect("Failed to save the modified PDF");
}
