#!/usr/bin/env python3
"""Compare bundled CP437 bitmaps with pinned, licensed pcface source maps.

Requires network access to fetch the pinned source revision. No generated files
are written; a successful run proves that the checked-in glyph bytes are either
identical to the source or differ only by the listed box-joining edits.
"""

from hashlib import sha256
from pathlib import Path
import re
from urllib.request import urlopen


REVISION = "8629ba46b73f58ac88cacbe8e77c6e9975dd221c"
FONT_RS = Path(__file__).resolve().parents[1] / "crates/framebuffer/src/font.rs"
EGA_BIN = Path(__file__).resolve().parents[1] / "crates/framebuffer/assets/ega-8x14.cp437.bin"

# The current VGA table joins horizontal box-drawing strokes at scanline 8.
# Keys are CP437 codepoints; values are (source byte, checked-in byte).
VGA_BOX_EDITS = {
    0xB4: (0x18, 0xF8),
    0xBF: (0x18, 0xF8),
    0xC0: (0x00, 0x1F),
    0xC1: (0x00, 0xFF),
    0xC2: (0x18, 0xFF),
    0xC3: (0x18, 0x1F),
    0xC4: (0x00, 0xFF),
    0xC5: (0x18, 0xFF),
    0xD9: (0x00, 0xF8),
    0xDA: (0x18, 0x1F),
}


def require(condition: bool, message: str) -> None:
    if not condition:
        raise ValueError(message)


def rust_table(source: str, name: str, rows: int) -> bytes:
    body = source.split(f"pub static {name}:", 1)[1].split("= [", 1)[1].split("];", 1)[0]
    body = "\n".join(line.split("//", 1)[0] for line in body.splitlines())
    data = bytes(int(value, 16) for value in re.findall(r"0x([0-9a-fA-F]{2})", body))
    require(len(data) == 256 * rows, f"{name}: expected {256 * rows} bytes, got {len(data)}")
    return data


def pcface_map(directory: str, rows: int) -> bytes:
    url = (
        f"https://raw.githubusercontent.com/susam/pcface/{REVISION}/out/"
        f"{directory}/fontmap.js"
    )
    source = urlopen(url, timeout=20).read().decode("utf-8")
    glyphs = []
    for line in source.splitlines():
        match = re.search(r": \[(.*?)\], // .*?\((\d+)\)", line)
        if match:
            index = int(match.group(2))
            require(index == len(glyphs), f"{directory}: unexpected glyph index {index}")
            glyph = bytes(
                int(value, 16)
                for value in re.findall(r"0x([0-9a-fA-F]{2})", match.group(1))
            )
            require(len(glyph) == rows, f"{directory}: glyph {index} has {len(glyph)} rows")
            glyphs.append(glyph)
    require(len(glyphs) == 256, f"{directory}: expected 256 glyphs, got {len(glyphs)}")
    return b"".join(glyphs)


def main() -> None:
    source = FONT_RS.read_text(encoding="utf-8")
    bios = rust_table(source, "CP437_FONT_8X8", 8)
    bios_upstream = pcface_map("oldschool-bios-8x8", 8)
    require(bios == bios_upstream, "CP437 8x8 differs from pinned BIOS source")

    vga = rust_table(source, "CP437_FONT_8X16", 16)
    vga_upstream = bytearray(pcface_map("oldschool-vga-8x16", 16))
    for codepoint, (before, after) in VGA_BOX_EDITS.items():
        offset = codepoint * 16 + 8
        require(vga_upstream[offset] == before, f"VGA source differs at codepoint {codepoint:02X}")
        vga_upstream[offset] = after
    require(vga == vga_upstream, "CP437 8x16 differs beyond documented box edits")

    ega = EGA_BIN.read_bytes()
    require(ega == pcface_map("oldschool-ega-8x14", 14), "EGA 8x14 differs from source")

    for name, data in (("CP437 8x8", bios), ("CP437 8x16", vga), ("EGA 8x14", ega)):
        print(f"{name}: {len(data)} bytes verified, SHA-256 {sha256(data).hexdigest()}")
    print(f"VGA box-joining edits: {len(VGA_BOX_EDITS)} scanline bytes")


if __name__ == "__main__":
    main()
