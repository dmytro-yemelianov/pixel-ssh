# EGA 8×14 bitmap attribution

`ega-8x14.cp437.bin` contains 256 CP437 glyphs, 14 bytes per glyph, in code-page order. Each byte is one 8-pixel scanline, most significant bit first.

Source: the `oldschool-ega-8x14/fontmap.js` bitmap in [Susam Pal's pcface project](https://github.com/susam/pcface/blob/8629ba46b73f58ac88cacbe8e77c6e9975dd221c/out/oldschool-ega-8x14/fontmap.js), generated from VileR's Oldschool PC Font Pack v2.2, IBM EGA 8×14. The bitmap was converted to this raw CP437 ordered format without editing glyph pixels.

The bitmap is available under the [Creative Commons Attribution-ShareAlike 4.0 International License](https://creativecommons.org/licenses/by-sa/4.0/). The source project's [license section](https://github.com/susam/pcface/blob/8629ba46b73f58ac88cacbe8e77c6e9975dd221c/README.md#license) explicitly lists `out/oldschool-ega-8x14/` under that license. The Rust rendering code remains MIT licensed.
