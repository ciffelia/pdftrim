# pdfcrop

[![CI status](https://github.com/ciffelia/pdftrim/actions/workflows/ci.yaml/badge.svg)](https://github.com/ciffelia/pdftrim/actions/workflows/ci.yaml)

Removes white margins from PDF files.

This tool uses Ghostscript to calculate the bounding boxes for each page in the input PDF file. It then generates an output PDF file with updated CropBox values to remove white margins.

While similar to the pdfcrop script in the TeX Live distribution, this tool offers a key advantage: it maintains minimal output file sizes. The traditional pdfcrop processes PDFs by importing them into TeX documents and converting them back to PDF using pdfTeX, XeTeX, or LuaTeX, which often results in significantly larger files. In contrast, this tool directly modifies the PDF dimension data, avoiding file size inflation.
