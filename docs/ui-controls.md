# Screen and control specification

The app has three independent display choices:

| Control | Changes | Does not change |
| --- | --- | --- |
| System (`S`) | Pixel resolution and character grid | Color palette or interface theme |
| Color (`C`) | The RGB values used by the framebuffer | Resolution, font, or layout |
| Theme (`T`) | Interface font and machine-specific styling | Resolution or RGB palette |

Visual effects (`V`) are a fourth, independent layer. Its menu shows the selected preset and the current value of each property; changing one property marks the preset as custom.

## Projects and navigation

The project screen begins with three destinations, followed by a divider and numbered portfolio entries:

```text
[00] CV
[@@] Contacts
[!!] About
===========
[01] RAPS
```

The footer contains `Prj Hlp Sys Col Thm Qut`, in that order. `P`, `H`, `S`, `C`, `T`, and `Q` invoke those controls; lowercase letters work where they do not conflict with contextual navigation. On the plain Projects list, `h` also opens Help. In a project detail, `h`/`l` or Left/Right change projects. `Qut` closes a dialog or detail first; from the plain Projects list it opens the screensaver in the browser or ends the SSH session in the terminal. Numbers do not select global destinations. Up/Down and `j`/`k` navigate the focused list or menu; Enter activates the selection; Escape closes a menu or returns to Projects. Menu-local numbered choices are allowed only while that menu is open. On touch screens, the corresponding buttons and direction pad perform the same actions.

| System resolution | Grid | Projects and footer | Settings controls |
| --- | --- | --- | --- |
| SVGA 800×600 | 100×37 | Wide list, six footer slots | Full selector labels and current-choice marker |
| SGA 640×480 | 80×30 | Standard list with extra rows, six footer slots | Full selector labels and current-choice marker |
| VGA 640×400 | 80×25 | Standard list, six footer slots | Full selector labels and current-choice marker |
| EGA 640×350 | 80×25, 14 px rows | Standard list, six footer slots | Full selector labels and current-choice marker |
| CGA 320×200 | 40×25 | Compact list, six abbreviated footer slots | One-column selector with current-choice marker |
| C64 320×200 | 40×25 | Compact list, six abbreviated footer slots | One-column selector with current-choice marker |
| Atari 320×192 | 40×24 | Compact list, six abbreviated footer slots | One-column selector with current-choice marker |
| ZX Spectrum 256×192 | 32×24 | Two-line project rows, six abbreviated footer slots | Narrow selector with current-choice marker; labels fit 32 columns |

Each System, Color, and Theme selector must show the current selection before and after a change. A theme can be paired with any palette and resolution. The portrait touch deck offers the same six footer actions, Visuals, preset buttons, arrows, Enter, and Escape.
