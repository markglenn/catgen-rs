CatGen-rs
=========

Recreation of the first app I ever sold, called [CatGen](https://github.com/markglenn/catgen).

CatGen was a DOS based self executable catalog generator for
[shareware](https://en.wikipedia.org/wiki/Shareware) vendors. Vendors could take
their existing text file based catalog and convert it into a full color, search
and printable, catalog that could then be shared on a floppy disk.

It was originally created for
[Microsoft QuickBASIC](https://en.wikipedia.org/wiki/QuickBASIC) on an old IBM
PC machine my parents let me use.

While the original code was lost, the example output file remains that contains
the actual rendering engine. This has been ported over and re-imagined in Rust
with some modern niceties such as resizable terminals and pre-compilation of the
document into [ANSI Escape Codes](https://en.wikipedia.org/wiki/ANSI_escape_code).
