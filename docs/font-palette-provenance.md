# Font and palette provenance audit

Audit date: 2026-09-28. This document records what the repository can reproduce from its tracked files. “Verified” means the repository contains a direct source reference and that the bundled bytes or RGB values can be checked against it. A historic name alone is not verification.

## Method

- Inspected every bundled bitmap in `crates/framebuffer/src/font.rs`, the EGA binary asset, the palette constructors in `crates/framebuffer/src/lib.rs`, and their Git history.
- Used only published technical material to describe the hardware constraints. It does not grant reuse rights for an asset.
- Did not replace any table. Unverified tables remain shipped pending a source with clear reuse rights and a byte-level comparison.

## Bundled font data

| Asset | Immediate source recorded in this repository | License | Bytes verified | Status and gap |
| --- | --- | --- | --- | --- |
| `EGA_FONT_8X14` (`assets/ega-8x14.cp437.bin`) | Susam Pal’s [`pcface` oldschool EGA 8×14 bitmap](https://github.com/susam/pcface/blob/8629ba46b73f58ac88cacbe8e77c6e9975dd221c/out/oldschool-ega-8x14/fontmap.js), converted to raw CP437 order without pixel edits | GPL-3.0-or-later or CC BY-SA 4.0; this crate selects CC BY-SA 4.0 as recorded in its [attribution](../crates/framebuffer/assets/README.md) and the [source license section](https://github.com/susam/pcface/blob/8629ba46b73f58ac88cacbe8e77c6e9975dd221c/README.md#license) | Yes: 256 × 14 bytes = 3584 bytes; SHA-256 `657ca6588b6bf729f0ed71a3d3c781a465bd30cdfb11eb5fc7e27594c0717ea1` | Sourced asset. The audit downloaded the pinned `fontlist.js`, extracted its 3584 hexadecimal row values, and found an exact byte-for-byte match. |
| `CP437_FONT_8X8` | Comment claims `nasinnia/data/ibm-ega-8x8.json`; no URL, revision, vendored input, or license is tracked. Public GitHub code and repository searches for that exact path/name returned no result during this audit. | Unknown | No | Encoding and dimensions are inspectable; historic origin and reuse rights are not. |
| `CP437_FONT_8X16` | Comment claims `nasinnia/data/ibm-vga-8x16.json`; no URL, revision, vendored input, or license is tracked. Public GitHub code and repository searches for that exact path/name returned no result during this audit. | Unknown | No | Encoding and dimensions are inspectable; historic origin and reuse rights are not. |
| `ZX_SPECTRUM_FONT_8X8` | None. Added as an inline table in commit `c0f0553a` | Unknown | No | 164 of 256 same-codepoint glyphs are byte-identical to `CP437_FONT_8X8`. That does not prove derivation, but it means an all-ROM claim is unsubstantiated. The repository records neither a ROM dump nor a licensed derivative. Sinclair documentation confirms the display is 32×24 with normal/bright digital color signals, but does not establish this table’s provenance ([service manual](https://spectrumforeveryone.com/wp-content/uploads/2017/11/ZX-Spectrum-128-Service-Manual.pdf)). |
| `C64_FONT_8X8` | None. Added as an inline table in commit `c0f0553a` | Unknown | No | 160 of 256 same-codepoint glyphs are byte-identical to `CP437_FONT_8X8`. The earlier PETSCII/901225-01 label had no character-ROM source or permission. The C64 has a 4 KB character-generator ROM ([VIC-II reference](https://github.com/martinpiper/C64Docs/blob/master/VIC/vic-ii.txt#L2364-L2369)); that fact does not verify these bytes. |
| `ATARI_FONT_8X8` | None. Added as an inline table in commit `c0f0553a` | Unknown | No | 155 of 256 same-codepoint glyphs are byte-identical to `CP437_FONT_8X8`. The earlier `REVAPAL ROM` label had no source, revision, or license. |

`pcface` documents its bitmap row format and CP437 ordering, which is the basis for checking the EGA conversion ([bitmap format](https://github.com/susam/pcface/blob/main/README.md#bitmap-format)).

## Named palettes

All values below are generated code, not separately licensed palette files. Their implementation is MIT except where the framebuffer crate’s package license also covers the bundled EGA asset. “RGB verified” refers to a chosen named source and settings; it does not merely mean that an RGBA literal exists in Rust.

| Theme | Immediate source recorded | RGB verified | Findings |
| --- | --- | --- | --- |
| Volkov Commander | None; handwritten application role colors | No | A DOS file-manager-inspired theme, not a recorded Volkov or Norton palette dump. |
| VGA Modern Dark | None; handwritten application role colors | No | Product theme; it does not claim a hardware palette. |
| EGA / CGA RGBI | None | No | The first sixteen entries are arranged for UI roles and do not preserve a canonical hardware color-index sequence. It should be described as RGBI-inspired. |
| ZX Spectrum | None | No | The hardware has 32×24 character display and digital RGB plus bright signals ([service manual](https://spectrumforeveryone.com/wp-content/uploads/2017/11/ZX-Spectrum-128-Service-Manual.pdf)); this code assigns those colors to application roles, with no measured or selected RGB reference. |
| Commodore 64 | Comment names “Pepto / Colodore” but provides no version, inputs, or settings | No | VIC-II has sixteen named color codes ([VIC-II reference](https://github.com/martinpiper/C64Docs/blob/master/VIC/vic-ii.txt#3.3-color-palette)), but RGB is an approximation of its analog signal. The Colodore author distinguishes its newer model from the older “Pepto” calculation and exposes adjustable output settings ([analysis](https://www.pepto.de/projects/colorvic/)). The code cannot be asserted to match either. |
| Atari 800 GTIA | None | No | GTIA generates luminance/chrominance, so there is no single hardware RGB table; conversion depends on the video standard and model ([Altirra hardware reference, appendix D](https://www.atari800xl.eu/docs/reference/altirra-hardware-reference-manual.pdf)). The initial UI colors and generated gradient are application approximations. |
| Amber CRT | None | No | Single-tint RGB design. No phosphor measurement, tube model, or color-management transform is recorded. |
| Green CRT | None | No | Single-tint RGB design. No IBM 5151/P1 phosphor measurement or color-management transform is recorded. |

The C64 and Atari source material explains why a literal RGB palette needs a declared conversion source and settings. It does not validate the application’s role ordering or dither patterns.

## Copy corrected by this audit

- `font.rs` now calls unverified inline tables “themed” or “CP437-ordered” instead of ROM-identical or hardware-authentic.
- `framebuffer/src/lib.rs` now describes palettes, dithers, and mouse pointers as application styling unless a source is recorded.
- The test formerly called `test_fonts_are_unique_and_authentic_per_system` only proves byte inequality, so it is now named for that actual assertion.
- The 80-, 40-, and 32-column About layouts now describe the unverified inline data as themed glyph matrices and CP437-ordered bitmaps.

## Before replacing any unverified asset

Record a stable URL and revision, the asset license and attribution requirements, the original byte encoding, a reproducible conversion command, and a byte-for-byte or RGB comparison result. Do not copy a ROM dump merely because it is available online.
