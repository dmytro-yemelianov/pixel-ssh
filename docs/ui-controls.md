# Screen and control specification

The `S` key cycles eight display systems. Each system applies its native resolution, character grid, font, and color palette. Visual effects (`V`) are adjustable separately. The Visuals menu shows the selected preset and each property value; changing one property marks the preset as custom.

## Projects and navigation

The project screen begins with three destinations, followed by a divider and numbered portfolio entries:

```text
[00] CV
[@@] Contacts
[!!] About
===========
[01] RAPS
```

The footer contains `Prj Hlp Sys Vis Qut`, in that order. `P`, `H`, `S`, `V`, and `Q` invoke those controls. `S` immediately cycles the eight systems. On the plain Projects list, `h` also opens Help. In a project detail, `h`/`l` or Left/Right change projects. `Qut` closes a dialog or detail first; from the plain Projects list it opens the screensaver in the browser or ends the SSH session in the terminal. Numbers do not select global destinations. Up/Down and `j`/`k` navigate the focused list or menu; Enter activates the selection; Escape closes a menu or returns to Projects. On touch screens, the corresponding buttons and direction pad perform the same actions.

| System resolution | Grid | Projects and footer | Settings controls |
| --- | --- | --- | --- |
| SVGA 800×600 | 100×37 | Wide list, five footer slots | Visuals preset and property values |
| SGA 640×480 | 80×30 | Standard list with extra rows, five footer slots | Visuals preset and property values |
| VGA 640×400 | 80×25 | Standard list, five footer slots | Visuals preset and property values |
| EGA 640×350 | 80×25, 14 px rows | Standard list, five footer slots | Visuals preset and property values |
| CGA 320×200 | 40×25 | Compact list, five abbreviated footer slots | Compact Visuals controls |
| C64 320×200 | 40×25 | Compact list, five abbreviated footer slots | Compact Visuals controls |
| Atari 320×192 | 40×24 | Compact list, five abbreviated footer slots | Compact Visuals controls |
| ZX Spectrum 256×192 | 32×24 | Two-line project rows, five abbreviated footer slots | Compact Visuals controls; labels fit 32 columns |

The portrait touch deck offers the same five footer actions, Visuals presets, arrows, Enter, and Escape. Long project titles and tags scroll horizontally within their assigned columns; article and detail content scroll vertically. Compact modal labels fit within their frame.

Project detail viewers fill the area above the bottom menu. SVGA uses its 100-column width for wrapped text and specification tables; the return and scroll controls follow the viewer's actual height.
