mod cli;

use std::{
    io::{BufRead, BufReader},
    process::{Command, Stdio},
};

use clap::Parser;
use log::debug;
use regex_lite::Regex;

fn main() {
    env_logger::init();

    let args = cli::Cli::parse();
    if let Some(shell) = args.generate_completion {
        let completion = match shell {
            clap_complete::Shell::Bash => include_str!("completion/bash"),
            clap_complete::Shell::Elvish => include_str!("completion/elvish"),
            clap_complete::Shell::Fish => include_str!("completion/fish"),
            clap_complete::Shell::PowerShell => include_str!("completion/powershell"),
            clap_complete::Shell::Zsh => include_str!("completion/zsh"),
            _ => unimplemented!(),
        };
        println!("{}", completion);

        return;
    }

    let gs_cmd = args
        .gscmd
        .as_deref()
        .or_else(|| find_ghostscript())
        .expect("Ghostscript not found in the system");
    debug!("Using Ghostscript command: {}", gs_cmd);

    let input_path = args.input.unwrap(); // input is required unless `--generate-completion` is present
    let input_path = if std::fs::metadata(&input_path).is_ok() {
        input_path
    } else {
        let with_extension = input_path.clone() + ".pdf";
        if std::fs::metadata(&with_extension).is_ok() {
            with_extension
        } else {
            panic!("Input file `{}' not found", input_path);
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

fn find_ghostscript() -> Option<&'static str> {
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
                return Some(candidate);
            }
        }
    }

    None
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
    debug!("Started Ghostscript");

    let stderr_reader = BufReader::new(child.stderr.take().unwrap());
    let (bboxes, hires_bboxes) = parse_ghostscript_output(stderr_reader);

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

fn parse_ghostscript_output(stderr: impl BufRead) -> (Vec<[f64; 4]>, Vec<[f64; 4]>) {
    let mut bboxes = Vec::new();
    let mut hires_bboxes = Vec::new();

    let re = Regex::new(
        r"(?-u)^%%(HiRes)?BoundingBox:\s*(-?[\.\d]+) (-?[\.\d]+) (-?[\.\d]+) (-?[\.\d]+)",
    )
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

    (bboxes, hires_bboxes)
}

fn crop_pdf(input_path: &str, output_path: &str, crop_boxes: &[[f64; 4]]) {
    let mut doc = lopdf::Document::load(input_path).expect("Failed to load the PDF");
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

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;

    #[test]
    fn test_parse_ghostscript_output_empty() {
        let input = "";
        let (bboxes, hires_bboxes) = parse_ghostscript_output(Cursor::new(input));
        assert!(bboxes.is_empty());
        assert!(hires_bboxes.is_empty());
    }

    #[test]
    fn test_parse_ghostscript_output_single_bbox() {
        let input = "%%BoundingBox: 10 20 30 40\n";
        let (bboxes, hires_bboxes) = parse_ghostscript_output(Cursor::new(input));
        assert_eq!(bboxes, vec![[10.0, 20.0, 30.0, 40.0]]);
        assert!(hires_bboxes.is_empty());
    }

    #[test]
    fn test_parse_ghostscript_output_single_hires_bbox() {
        let input = "%%HiResBoundingBox: 10.5 20.5 30.5 40.5\n";
        let (bboxes, hires_bboxes) = parse_ghostscript_output(Cursor::new(input));
        assert!(bboxes.is_empty());
        assert_eq!(hires_bboxes, vec![[10.5, 20.5, 30.5, 40.5]]);
    }

    #[test]
    fn test_parse_ghostscript_output_multiple_mixed() {
        let input = r#"Processing pages 1 through 2.
Page 1
%%BoundingBox: 133 179 478 678
%%HiResBoundingBox: 133.919996 179.045995 477.395985 677.015979
Page 2
%%BoundingBox: 133 525 478 715
%%HiResBoundingBox: 133.343996 525.869984 477.395985 714.023978
"#;
        let (bboxes, hires_bboxes) = parse_ghostscript_output(Cursor::new(input));
        assert_eq!(
            bboxes,
            vec![[133.0, 179.0, 478.0, 678.0], [133.0, 525.0, 478.0, 715.0]]
        );
        assert_eq!(
            hires_bboxes,
            vec![
                [133.919996, 179.045995, 477.395985, 677.015979],
                [133.343996, 525.869984, 477.395985, 714.023978]
            ]
        );
    }

    #[test]
    fn test_parse_ghostscript_output_actual() {
        let input = r#"GPL Ghostscript 9.55.0 (2021-09-27)
Copyright (C) 2021 Artifex Software, Inc.  All rights reserved.
This software is supplied under the GNU AGPLv3 and comes with NO WARRANTY:
see the file COPYING for details.
Processing pages 1 through 1.
Page 1
Loading NimbusSans-Regular font from /usr/share/ghostscript/9.55.0/Resource/Font/NimbusSans-Regular... 4469404 2930106 4289320 2951995 5 done.
%%BoundingBox: 101 99 401 376
%%HiResBoundingBox: 101.999528 99.449997 400.508988 375.515989
"#;
        let (bboxes, hires_bboxes) = parse_ghostscript_output(Cursor::new(input));
        assert_eq!(bboxes, vec![[101.0, 99.0, 401.0, 376.0]]);
        assert_eq!(
            hires_bboxes,
            vec![[101.999528, 99.449997, 400.508988, 375.515989],]
        );
    }
}
