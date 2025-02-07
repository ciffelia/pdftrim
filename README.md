# pdftrim

[![CI status][ci badge]][ci link]
[![crates.io][crates.io badge]][crates.io link]
[![Apache 2.0 or MIT Licenses][license badge]][license link]

Removes white margins from PDF files without inflating file sizes.

pdftrim is a tool to trim white margins from PDF files. It is designed to be fast and efficient, producing minimal output file sizes.

This tool uses Ghostscript to calculate the bounding boxes for each page in the input PDF file. It then generates an output PDF file with updated CropBox values to remove white margins.

While similar to the pdfcrop script in the TeX Live distribution, this tool offers a key advantage: it maintains minimal output file sizes. The traditional pdfcrop processes PDFs by importing them into TeX documents and converting them back to PDF using pdfTeX, XeTeX, or LuaTeX, which often results in significantly larger files. In contrast, this tool directly modifies the PDF dimension data, avoiding file size inflation.

## Installation

**Prebuilt binaries are available for Linux, macOS, and Windows on the [latest release](https://github.com/ciffelia/pdftrim/releases/latest) page.**

You need to have [Ghostscript](https://www.ghostscript.com/releases/gsdnld.html) installed on your system.

```sh
brew install ghostscript      # macOS
sudo apt install ghostscript  # Ubuntu
sudo yum install ghostscript  # CentOS
sudo dnf install ghostscript  # Fedora
```

### Alternative installation methods

#### Cargo

With Rust installed:

```sh
cargo install pdftrim
```

You need to have Ghostscript installed on your system.

#### Cargo Binstall

With Rust and [cargo-binstall](https://github.com/cargo-bins/cargo-binstall) installed:

```sh
cargo binstall pdftrim
```

You need to have Ghostscript installed on your system.

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

  -V, --version
          Print version

Verbose output can be enabled by setting the `RUST_LOG` environment variable to `debug`.
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

[ci badge]: https://github.com/ciffelia/pdftrim/actions/workflows/ci.yaml/badge.svg
[ci link]: https://github.com/ciffelia/pdftrim/actions/workflows/ci.yaml

[crates.io badge]: https://img.shields.io/crates/v/pdftrim?logo=rust
[crates.io link]: https://crates.io/crates/pdftrim

[license badge]: https://img.shields.io/badge/license-Apache--2.0_OR_MIT-blue
[license link]: #license
