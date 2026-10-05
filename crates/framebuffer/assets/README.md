# Bitmap font attribution

The Oldschool PC bitmaps below are from VileR's Oldschool PC Font Pack v2.2, converted to CP437 scanline arrays by [Susam Pal's pcface project](https://github.com/susam/pcface). The [source license section](https://github.com/susam/pcface/blob/8629ba46b73f58ac88cacbe8e77c6e9975dd221c/README.md#license) offers these bitmap directories under GPL v3 or [Creative Commons Attribution-ShareAlike 4.0 International](https://creativecommons.org/licenses/by-sa/4.0/); this crate uses the CC BY-SA 4.0 option. The Rust rendering logic remains MIT licensed.

## CP437 8×8 and 8×16

The inline `CP437_FONT_8X8` table in `src/font.rs` exactly matches the pinned [Oldschool BIOS 8×8 bitmap](https://github.com/susam/pcface/blob/8629ba46b73f58ac88cacbe8e77c6e9975dd221c/out/oldschool-bios-8x8/fontmap.js) in CP437 order.

The inline `CP437_FONT_8X16` table matches the pinned [Oldschool VGA 8×16 bitmap](https://github.com/susam/pcface/blob/8629ba46b73f58ac88cacbe8e77c6e9975dd221c/out/oldschool-vga-8x16/fontmap.js), except for ten scanline-8 bytes in box-drawing glyphs `B4`, `BF`, `C0`–`C5`, `D9`, and `DA`. Those bytes were changed to join horizontal strokes. Run `python3 tools/verify_font_provenance.py` from the repository root to fetch the pinned inputs and compare every byte.

## EGA 8×14

`ega-8x14.cp437.bin` contains 256 CP437 glyphs, 14 bytes per glyph, in code-page order. Each byte is one 8-pixel scanline, most significant bit first.

Source: the [Oldschool EGA 8×14 bitmap](https://github.com/susam/pcface/blob/8629ba46b73f58ac88cacbe8e77c6e9975dd221c/out/oldschool-ega-8x14/fontmap.js). It was converted to this raw CP437 ordered format without editing glyph pixels.

The EGA bitmap is used under the same CC BY-SA 4.0 option described above. The verifier also compares all 3584 EGA bytes to the pinned source.

## Cyrillic 8×16, 8×14 and 8×8

`cyrillic-8x16.bin`, `cyrillic-8x14.bin` and `cyrillic-8x8.bin` contain real bitmaps for the 94 Cyrillic letters Ukrainian and Russian need: Ё–Ќ, Ў–я, ё–ќ, ў–џ, and Ґ ґ. pcface covers only CP437, so these come directly from the Unicode-extended bitmap fonts of the same VileR Oldschool PC Font Pack v2.2:
- `BmPlus_IBM_VGA_8x16.otb` for 8×16;
- `BmPlus_IBM_EGA_8x14.otb` for 8×14;
- `BmPlus_IBM_BIOS.otb` for 8×8.

The pack is licensed under [CC BY-SA 4.0](https://creativecommons.org/licenses/by-sa/4.0/) (its `LICENSE.TXT`). Glyph `i` is `CYRILLIC_CODEPOINTS[i]` in `src/font.rs`, with one byte per scanline, most significant bit first. Pixels were not edited.

`python3 tools/extract_cyrillic_fonts.py --check` downloads the pinned pack, checks its SHA-256 and regenerates every byte. The script needs `pip install freetype-py`.

The same fonts were checked against the existing CP437 tables:
- all 95 printable ASCII glyphs of VGA 8×16 and EGA 8×14 are byte-identical (EGA after skipping the two padding rows of the OTB cell);
- 94 of 95 match for BIOS 8×8; only `;` sits one row lower in pcface.
