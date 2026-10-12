# The brand layer

Atelier Prism is EffectCraft with a different name on the door. We keep the change small on purpose:
upstream lands many changes a day, and every file we touch is a file that can conflict when we pull
them in. So:

**Renamed (what a person sees):** the window title, menus (including the macOS app menu and Help),
About (Atelier mark, the line on where Prism comes from, the Nation link first), the header's
"Nation" button and links, the Home screen's links and "More Atelier apps" (Pigment, Quill, Reel,
Frequency, Lens, Seal, Press), messages in the UI crate, file dialogs, Windows file properties, the
MCP server title, installer and package names, vendor and links, the browser page and manifest, the
AppImage desktop entry, the demo project that opens at launch ("Atelier Prism Intro", its title
and lower third; `crates/engine/src/demo.rs`), and the app icon and favicon.

**Not renamed (machine names):** crate names (`effectcraft-*`), the binaries (`effectcraft`,
`effectcraft-cli`), command ids (`help.discord` opens the Nation, `help.website` the Atelier suite,
`help.sibling` the other Atelier apps) and automation ids, the `ai.storyteller.effectcraft` app id
and icon file names in packaging, the `.ecproj`/`.ecprojx` formats and their generator and XML root,
folders on disk (the auto-save folder, the presets folder, the settings and log folders), the
default shortcut preset's name, environment variables
(`EFFECTCRAFT_*`), and the docs under `docs/`.

## Where the name lives

| Place | File |
|---|---|
| Links and the other Atelier apps | `crates/engine/src/links.rs`, `crates/engine/src/commands/help.rs` |
| Menu tree (macOS app menu, Help) | `crates/engine/src/menus.rs`, `crates/engine/src/commands/frontend.rs` |
| Menu translations | rows in `crates/ui-egui/src/i18n.rs` (six languages) |
| Panel translations | rows in `crates/ui-egui/src/i18n/ui.rs` (four languages) |
| English UI strings | literal `"Atelier Prism"` in `crates/ui-egui/src` and `apps/` |
| Header button, Atelier mark | `crates/ui-egui/src/header.rs` (`paint_logo` draws `assets/brand/atelier-mark.png`) |
| About, Home | `crates/ui-egui/src/panels/dialogs.rs`, `panels/home.rs` |
| MCP | `crates/automation/src/server.rs` |
| Windows file properties | `apps/effectcraft/build.rs` |
| macOS bundle | `packaging/macos/Info.plist.in` (display name, copyright, document types; identifier unchanged) |
| Windows installer | `packaging/windows/effectcraft.wxs` (names, manufacturer, links, install folder; upgrade code unchanged) and `test-msi.ps1` |
| Linux desktop entry, AppStream, nfpm, AppImage | `packaging/linux/ai.storyteller.effectcraft.*`, `nfpm.yaml`, `apps/effectcraft/src/appimage.rs` |
| Browser build | `apps/effectcraft-web/web/index.html`, `manifest.webmanifest`, `favicon.svg` |
| README and NOTICE | README: our header above upstream's feature sections; NOTICE: ours on top of theirs, kept on merge by `.gitattributes` |
| App icon | `assets/app-icon/` (ours; every file has a `.attribution` sidecar and a row in `ATTRIBUTION.md`) |

Still upstream's name, for later passes: messages from the engine crates (project, template and
preset errors), and the `docs/` manuals. For the packaging pass (real installers): the macOS `.app`
bundle and DMG names (`packaging/macos/package.sh`), the DMG background, and the package scripts'
file names.
