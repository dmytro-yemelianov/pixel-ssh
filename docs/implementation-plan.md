# Compact display and provenance implementation plan

1. **ZX Projects:** Change the 32-column project list to use distinct title and tag rows. Update visible-row counts, selection background, scroll limits, and pointer hit mapping together. Keep all 16 projects reachable.
2. **ZX CV:** Edit the 32-column CV content and, if needed, article spacing so roles and sections scan cleanly without crossing the border or scrollbar. Verify scrolling reaches the last line.
3. **Provenance:** Trace the bundled bitmap fonts and named palettes to source material. Record evidence and license status, soften unsupported comments and public copy, and identify any unresolved items.
4. **Integration:** Review the independent changes for inconsistent counts or shared copy, add focused regression checks, rebuild WebAssembly, inspect browser captures at desktop ZX and portrait sizes, and fix any defects found.

The first three steps can proceed in parallel. The integration step depends on their results. Push and GitHub CI are excluded by request.

## Local implementation result

- ZX Projects uses eight two-row items per window; title and tags have separate horizontal scroll regions. The selection, pointer mapping, wheel movement, and scroll limits use the same row count.
- ZX CV uses short role sections and blank separators. Browser `End` reaches the final contact line; keyboard Home/End/PageUp/PageDown are now passed through to the Rust app.
- The provenance audit is in `font-palette-provenance.md`. Unsupported original-ROM and exact-hardware claims were removed from code and About copy. The CP437 8×8, VGA 8×16, and EGA 8×14 bitmaps now have pinned source comparisons; source and reuse rights for the ZX, C64, and Atari themed tables remain documented gaps.
- Verification: 49 workspace tests, formatting, strict Clippy, WASM target check, and release WebAssembly build passed. ZX Projects and CV were captured at 390×844 and 1280×900; keyboard End and a touch drag were exercised in the portrait browser. The browser console was empty.
