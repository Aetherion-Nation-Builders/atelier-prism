# Atelier Prism app icon

**The hidden sun behind a cut crystal**: an ivory disc rising behind an ochre crystal, its table and
facets cut in black, with an ivory glint, on a volcanic black field. Three colours, from the Nation's
palette: volcanic black `#141210`, ivory `#efe6d2`, ochre `#d9a441`.

**Tile:** 512 × 512 viewBox, rounded square `rx=112`, as all the Atelier icons. The macOS files add
Apple's 824/1024 margin (`effectcraft-macos-512.png`, `.icns`).

**Provenance:** drawn for Atelier Prism by hand as SVG (2026). It replaces EffectCraft's unicorn icon,
which stays in EffectCraft's own repository. Every file here has a `.attribution` sidecar and a row in
`ATTRIBUTION.md`. Licence: `LICENSE.txt` (MIT OR Apache-2.0). The file names keep their upstream
`effectcraft` names so packaging stays untouched; `effectcraft-small.svg` is the same artwork, and the
web favicon (`apps/effectcraft-web/web/favicon.svg`) is a copy of it.

**Regenerating:** `packaging/icons.sh` (needs resvg; iconutil on macOS for the `.icns`). The committed
renders were made from the same SVG with cairosvg and Pillow.
