/// Removes white margins from PDF files without inflating file sizes.
///
/// pdftrim is a tool to trim white margins from PDF files. It is designed to
/// be fast and efficient, producing minimal output file sizes.
///
/// This tool uses Ghostscript to calculate the bounding boxes for each page in
/// the input PDF file. It then generates an output PDF file with updated
/// CropBox values to remove white margins.
///
/// While similar to the pdfcrop script in the TeX Live distribution, this tool
/// offers a key advantage: it maintains minimal output file sizes. The
/// traditional pdfcrop processes PDFs by importing them into TeX documents and
/// converting them back to PDF using pdfTeX, XeTeX, or LuaTeX, which often
/// results in significantly larger files. In contrast, this tool directly
/// modifies the PDF dimension data, avoiding file size inflation.
#[derive(clap::Parser)]
#[command(
    version,
    after_long_help = "Verbose output can be enabled by setting the `RUST_LOG` environment \
                       variable to `debug`."
)]
pub struct Cli {
    /// The input PDF file to crop.
    ///
    /// The `.pdf` extension is optional.
    #[arg(value_name = "input[.pdf]", value_hint = clap::ValueHint::FilePath, required_unless_present = "generate_completion")]
    pub input: Option<String>,

    /// The output PDF file to write.
    ///
    /// If not specified, the input file name is used with `-crop.pdf` appended.
    /// Existing files will be overwritten.
    #[arg(value_name = "output file", value_hint = clap::ValueHint::FilePath)]
    pub output: Option<String>,

    /// The Ghostscript command to use.
    ///
    /// If not specified, it will search for Ghostscript in the system.
    #[arg(long, value_name = "command", value_hint = clap::ValueHint::CommandName)]
    pub gscmd: Option<String>,

    /// If provided, outputs the completion file for given shell and exits.
    #[arg(long, value_enum, exclusive = true)]
    pub generate_completion: Option<clap_complete::Shell>,
}
