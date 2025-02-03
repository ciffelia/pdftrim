# pdfcrop

[![CI status](https://github.com/ciffelia/pdftrim/actions/workflows/ci.yaml/badge.svg)](https://github.com/ciffelia/pdftrim/actions/workflows/ci.yaml)

Removes white margins from PDF files without inflating file sizes.

pdfcrop is a tool to trim white margins from PDF files. It is designed to be fast and efficient, producing minimal output file sizes.

This tool uses Ghostscript to calculate the bounding boxes for each page in the input PDF file. It then generates an output PDF file with updated CropBox values to remove white margins.

While similar to the pdfcrop script in the TeX Live distribution, this tool offers a key advantage: it maintains minimal output file sizes. The traditional pdfcrop processes PDFs by importing them into TeX documents and converting them back to PDF using pdfTeX, XeTeX, or LuaTeX, which often results in significantly larger files. In contrast, this tool directly modifies the PDF dimension data, avoiding file size inflation.

## Usage

```
Usage: pdftrim <input[.pdf]> [output file]

Arguments:
  <input[.pdf]>
          The input PDF file to crop.
          
          The `.pdf` extension is optional.

  [output file]
          The output PDF file to write.
          
          If not specified, the input file name is used with `-crop.pdf` appended. Existing files will be overwritten.

Options:
  -h, --help
          Print help (see a summary with '-h')

Verbose output can be enabled by setting the `RUST_LOG` environment variable to `debug`.
```
