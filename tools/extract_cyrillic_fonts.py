#!/usr/bin/env python3
"""Extract Cyrillic bitmap glyphs from the pinned Oldschool PC Font Pack v2.2.

The CP437 tables come from pcface, which only covers CP437. The same font pack's
Unicode-extended "BmPlus" bitmap fonts contain Cyrillic. This script downloads
the pinned pack, checks its SHA-256, reads the three bitmap fonts with FreeType
and writes one raw glyph file per size, in the codepoint order of
CYRILLIC_CODEPOINTS (8-pixel rows, most significant bit first).

Usage:
  python3 tools/extract_cyrillic_fonts.py          # write the assets
  python3 tools/extract_cyrillic_fonts.py --check  # verify the checked-in assets

Requires network access and `pip install freetype-py`.
"""

from hashlib import sha256
from io import BytesIO
from pathlib import Path
import sys
import tempfile
from urllib.request import Request, urlopen
import zipfile

import freetype

PACK_URL = "https://int10h.org/oldschool-pc-fonts/download/oldschool_pc_font_pack_v2.2_FULL.zip"
PACK_SHA256 = "21b3c0a3770ef0afc46564760613d7b078f4fcc9ed93db4b829a440b68822e08"
ASSETS = Path(__file__).resolve().parents[1] / "crates/framebuffer/assets"

# Every Cyrillic letter Ukrainian and Russian text needs, in Unicode order:
# Ё–Ќ, Ў–я, ё–ќ, ў–џ, and Ґ ґ. All are present in all three source fonts.
CYRILLIC_CODEPOINTS = (
    list(range(0x0401, 0x040D)) + list(range(0x040E, 0x0450))
    + list(range(0x0451, 0x045D)) + list(range(0x045E, 0x0460))
    + [0x0490, 0x0491]
)

# (pack member, output file, glyph rows, rows to skip at the top of the OTB cell)
# The skip offsets align the glyphs with the existing CP437 tables: with them,
# all 95 printable ASCII glyphs of VGA 8x16 and EGA 8x14 are byte-identical to
# the bundled tables, and 94 of 95 for BIOS 8x8 (only ';' differs by one row).
FONTS = [
    ("otb - Bm (linux bitmap)/BmPlus_IBM_VGA_8x16.otb", "cyrillic-8x16.bin", 16, 0),
    ("otb - Bm (linux bitmap)/BmPlus_IBM_EGA_8x14.otb", "cyrillic-8x14.bin", 14, 2),
    ("otb - Bm (linux bitmap)/BmPlus_IBM_BIOS.otb", "cyrillic-8x8.bin", 8, 0),
]


def glyph_rows(path: str) -> tuple[dict[int, list[int]], int]:
    face = freetype.Face(path)
    face.select_size(0)
    cell_h = face.available_sizes[0].height
    ascender = face.size.ascender >> 6
    out: dict[int, list[int]] = {}
    cp, gi = face.get_first_char()
    while gi:
        face.load_glyph(gi, freetype.FT_LOAD_RENDER | freetype.FT_LOAD_TARGET_MONO)
        slot = face.glyph
        bm = slot.bitmap
        rows = [0] * cell_h
        top = ascender - slot.bitmap_top
        for y in range(bm.rows):
            for x in range(bm.width):
                if bm.buffer[y * bm.pitch + x // 8] & (0x80 >> (x % 8)):
                    cy, cx = top + y, slot.bitmap_left + x
                    if 0 <= cy < cell_h and 0 <= cx < 8:
                        rows[cy] |= 0x80 >> cx
        out[cp] = rows
        cp, gi = face.get_next_char(cp, gi)
    return out, cell_h


def build() -> dict[str, bytes]:
    # int10h.org rejects the default Python User-Agent with HTTP 403.
    data = urlopen(Request(PACK_URL, headers={"User-Agent": "pixel-ssh-font-tools/1.0"})).read()
    digest = sha256(data).hexdigest()
    if digest != PACK_SHA256:
        raise SystemExit(f"font pack SHA-256 mismatch: {digest}")
    pack = zipfile.ZipFile(BytesIO(data))
    result = {}
    with tempfile.TemporaryDirectory() as tmp:
        for member, out_name, rows, skip in FONTS:
            path = Path(tmp) / Path(member).name
            path.write_bytes(pack.read(member))
            glyphs, cell_h = glyph_rows(str(path))
            if skip + rows > cell_h:
                raise SystemExit(f"{member}: cell {cell_h} too small")
            blob = bytearray()
            for cp in CYRILLIC_CODEPOINTS:
                if cp not in glyphs:
                    raise SystemExit(f"{member}: missing U+{cp:04X}")
                blob += bytes(glyphs[cp][skip:skip + rows])
            result[out_name] = bytes(blob)
    return result


def main() -> None:
    built = build()
    check = "--check" in sys.argv
    for name, blob in built.items():
        target = ASSETS / name
        if check:
            if target.read_bytes() != blob:
                raise SystemExit(f"{name} differs from the pinned source")
            print(f"{name}: {len(blob)} bytes match, SHA-256 {sha256(blob).hexdigest()}")
        else:
            target.write_bytes(blob)
            print(f"wrote {name}: {len(blob)} bytes, SHA-256 {sha256(blob).hexdigest()}")
    print(f"{len(CYRILLIC_CODEPOINTS)} codepoints")


if __name__ == "__main__":
    main()
