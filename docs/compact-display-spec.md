# Compact display and asset provenance specification

## Goal

Make the native 32-column ZX layout readable on the Projects and CV screens, with usable keyboard and touch scrolling. Audit the font and palette claims that appear in code and user-facing copy. Preserve the independent resolution and color theme choices; Volkov Commander remains an optional palette.

## Compact layout requirements

- A project list item must have a visible selection/index and a readable title. Tags must have their own clearly separated area. No text or background from one item may overlap the next, the bottom status line, or navigation.
- Long titles and tag lists may scroll horizontally, but the current item must retain its identity while the marquee moves. All 16 projects must remain reachable with keyboard, wheel, and touch input.
- The 32-column CV article must use short, meaningful lines and visible separation between roles and sections. Preserve the factual content in the existing CV; do not add unsupported achievements or metrics.
- The article body must stay between its border and scrollbar, and the final line must be reachable. Close, scroll, and tab actions must continue to work in 32-, 40-, and 80-column layouts.
- Portrait controls must stay within a 390×844 CSS viewport with no horizontal page overflow. The adaptive 40-column portrait default is intentional; an explicit ZX resolution must remain ZX.

## Asset and copy requirements

- For each bundled font and named hardware palette, record its immediate source, license if known, and whether the bytes or RGB values were verified against that source.
- Use neutral descriptions when provenance or hardware fidelity is unverified. Keep the existing EGA 8×14 asset attribution and licensing intact.
- Correct user-facing claims that the audit disproves. Do not replace asset data with a source of unclear reuse rights.

## Acceptance

- Rust formatting, workspace tests, and strict Clippy pass. Tests cover 32-column bounds, reachable scrolling, and selection/input mapping where behavior changes.
- Rebuild the tracked WebAssembly bundle. Capture Projects and CV in native ZX mode and in a portrait viewport; inspect both screens and controls. Check the browser console.
- Document remaining provenance gaps explicitly rather than presenting an unverified authenticity claim as fact.
