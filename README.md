CatGen-rs
=========

[![Screenshot](https://github.com/markglenn/catgen-rs/raw/main/media/screenshot.png)](#features)

CatGen-rs is a recreation of the first app I ever sold, called
[CatGen](https://github.com/markglenn/catgen).

CatGen was a DOS based self executable catalog generator for
[shareware](https://en.wikipedia.org/wiki/Shareware) vendors. Vendors could take
their existing text file based catalog and convert it into a full color,
searchable and printable, catalog that could then be shared on a floppy disk.

The output document was similar to a text based PDF engine using a custom markup
language since markup languages at the time were in their infancy and not really
open.

CatGen v1.0 was originally created for
[IBM Advanced BASIC](https://en.wikipedia.org/wiki/IBM_BASIC#IBM_Advanced_BASIC)
while later versions were ported to
[Microsoft QuickBASIC](https://en.wikipedia.org/wiki/QuickBASIC), mostly on an
old IBM PC machine my parents let me use.

While the original generator code was lost, the example output file remains that
contains the actual rendering engine. This has been ported over and re-imagined
in Rust with some modern niceties such as resizable terminals and
pre-compilation of the document into
[ANSI Escape Codes](https://en.wikipedia.org/wiki/ANSI_escape_code).
