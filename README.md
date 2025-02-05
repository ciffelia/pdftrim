# pdftrim

[![CI status](https://github.com/ciffelia/pdftrim/actions/workflows/ci.yaml/badge.svg)](https://github.com/ciffelia/pdftrim/actions/workflows/ci.yaml)

Removes white margins from PDF files without inflating file sizes.

pdftrim is a tool to trim white margins from PDF files. It is designed to be fast and efficient, producing minimal output file sizes.

This tool uses Ghostscript to calculate the bounding boxes for each page in the input PDF file. It then generates an output PDF file with updated CropBox values to remove white margins.

While similar to the pdfcrop script in the TeX Live distribution, this tool offers a key advantage: it maintains minimal output file sizes. The traditional pdfcrop processes PDFs by importing them into TeX documents and converting them back to PDF using pdfTeX, XeTeX, or LuaTeX, which often results in significantly larger files. In contrast, this tool directly modifies the PDF dimension data, avoiding file size inflation.

## Usage

```
Usage: pdftrim [OPTIONS] [input[.pdf]] [output file]

Arguments:
  [input[.pdf]]
          The input PDF file to crop.
          
          The `.pdf` extension is optional.

  [output file]
          The output PDF file to write.
          
          If not specified, the input file name is used with `-crop.pdf` appended. Existing files will be overwritten.

Options:
      --gscmd <command>
          The Ghostscript command to use.
          
          If not specified, it will search for Ghostscript in the system.

      --generate-completion <GENERATE_COMPLETION>
          If provided, outputs the completion file for given shell and exits
          
          [possible values: bash, elvish, fish, powershell, zsh]

  -h, --help
          Print help (see a summary with '-h')

Verbose output can be enabled by setting the `RUST_LOG` environment variable to `debug`.
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
