# Pocket Sheets — desktop app (Tauri 2)

A Tauri 2.x shell that turns the Pocket Sheets webapp into a fully functional
downloadable app for **macOS** (Apple Silicon arm64 + Intel x86_64, `.dmg`) and
**Windows** (x86_64, `.msi` + `.exe` NSIS installer). Built on GitHub Actions
from this project; nothing is built on the developer's machine.

## Feature parity — the desktop app IS the webapp

The desktop app wraps the **identical** Pocket Sheets HTML file (currently v1.1).
It is not a cut-down build. Same file, same features, same storage behaviour,
same capability:

- A grid (100×26, grows at the edges) with click/double-click/type editing,
  Enter/Tab/arrows, column resize, insert/delete rows and columns.
- A formula engine (SUM, AVERAGE, COUNT, COUNTA, MIN, MAX, IF, IFERROR,
  CONCATENATE, VLOOKUP, COUNTIF, SUMIF, ROUND, ABS, UPPER, LOWER, TRIM, LEN,
  TODAY, NOW; arithmetic, comparisons, cross-sheet refs; dependency-graph
  cycle detection → `#CIRC!`).
- Sheet tabs (add / rename / delete), sort A–Z/Z–A, filter by value,
  find, undo/redo (50 snapshots), formatting (bold/italic/fill/text color/
  number format/borders/merge), Cards view, Help.
- Import/export: CSV (file + paste), `.xlsx` (pure-JS ZIP/DEFLATE engine),
  and the open Weave (`.weave`, `kind: sheets`) format.
- The storage engine: IndexedDB is the primary store, localStorage is the
  fallback with a visible "Limited storage" badge, failed saves retry once
  then say so. The Storage dialog shows store/usage/quota and exports a
  backup.

The native shell only **adds** two things:

1. **Native file dialogs.** In the desktop shell, import uses the native Open
   dialog and export uses the native Save As dialog (see "Why the shell
   needs native dialogs" below) instead of the browser file picker and
   download. The v1.1 app file routes through the shell's commands when it
   runs inside Tauri and falls back to the browser paths in a plain browser.
2. **Native storage with no purge risk.** Workbooks live in the desktop
   webview's own persistent data store on the machine — the OS-managed app
   data, not a browser's evictable web cache. Unlike the iPhone/iPad
   home-screen web app, which iOS can purge if unused, the desktop app's
   storage does not carry that risk. The same "export a backup (.weave)"
   habit is still the safety net.

## Why the shell needs native dialogs

Two webview gaps, both fixed in the v1.1 app file + this shell:

- **Import on macOS:** the webview's `<input type="file">` picker does not
  deliver the selected file in Tauri on macOS — the dialog opens, the user
  picks a file, and the input's `change` event never fires. Import would
  silently do nothing. (This is the Pocket Notes v37 lesson.) The v1.1 app
  file's `importViaTauri()` opens the native dialog and reads the file with
  the shell's `read_text_file` / `read_bytes_file` commands instead.
- **Export on Windows:** the webview silently drops the browser anchor
  download (`<a download>` with a blob URL) — WebView2 has no browser
  download manager to hand it to. The v1.1 app file's `downloadBytes()` /
  `downloadText()` open the native Save dialog and write with the shell's
  `save_text_file` / `save_bytes_file` commands instead. (This is the Pocket
  Notes v18 lesson.)

The shell-side half is in this project:

- `tauri-plugin-dialog` in `src-tauri/Cargo.toml`, initialised in
  `src/main.rs`, with the `dialog:default` permission in
  `src-tauri/capabilities/default.json`.
- The four Rust commands in `src/main.rs`: `read_text_file`,
  `read_bytes_file` (base64), `save_text_file`, `save_bytes_file` (base64).
  App-defined commands need no capability entry.
- `app.withGlobalTauri: true` in `tauri.conf.json`, so the shell injects
  `window.__TAURI__` (with the dialog plugin's JS under
  `window.__TAURI__.dialog`) into the page. Tauri 2 does **not** inject it by
  default — without this the live file could never detect the shell.

## Deliberately inert on desktop (not a loss)

- The PWA additions (manifest, service worker) are inert here: the service
  worker only registers over http(s), and under the desktop shell the page is
  served from the `tauri://` protocol, so registration is skipped. The
  desktop app does not need a service worker — its files are local and
  always available offline.
- "Add to Home Screen" / PWA install is not applicable — the desktop app IS
  the install.

## Installing on a Mac — the unsigned-app note

This app is not code-signed (no Apple Developer ID, $99/year). On first
launch macOS may say the app "is damaged and can't be opened". That is
Gatekeeper reacting to an unsigned app, not a damaged file. The fix (the
same one Pocket Notes uses):

1. Give Terminal Full Disk Access (System Settings → Privacy & Security →
   Full Disk Access → add Terminal).
2. Run: `xattr -cr /Applications/Pocket\ Sheets.app`
3. Open the app normally.

If the app is ever code-signed/notarized for distribution, this note goes
away.

## How to update the app

The live file `../pocket-sheets.html` is the single source of truth. Never
edit `dist/index.html` by hand — regenerate it from the live file:

```sh
cp ../pocket-sheets.html dist/index.html
sha256sum ../pocket-sheets.html dist/index.html   # the two hashes must match
```

Then bump `version` in `src-tauri/Cargo.toml` and in
`src-tauri/tauri.conf.json` to match the app version (currently `1.1.0` for
v1.1), and commit. The fork-before-overwrite rule applies to the live file:
fork the outgoing version into `../versions/` before editing it.

## How to build (GitHub Actions)

1. Create a new GitHub repository (pick any name — the name is yours to set;
   nothing in this project hard-codes one) and push the contents of this
   `desktop/` folder as the repository root, so the workflow lands at
   `.github/workflows/build.yml` in the repo.
2. Push to `main` (or run the workflow manually from the Actions tab).
   The workflow builds on GitHub's own runners — macOS for the two `.dmg`
   files (Apple Silicon arm64 and Intel x86_64) and Windows for the `.msi`
   and the NSIS `.exe` — uploads them as run artifacts
   (`pocket-sheets-aarch64-apple-darwin`, `pocket-sheets-x86_64-apple-darwin`,
   `pocket-sheets-x86_64-pc-windows-msvc`), and publishes them to the
   `v1.1.0` GitHub Release.
3. Download the installers from the Release.

Local builds need the Rust toolchain (`rustup`) and the Tauri CLI; the CI path
above is the intended one, since a Linux machine cannot produce the macOS
`.dmg` or the Windows installers.

## iOS is parked

There is deliberately no iOS target here. Publishing to the App Store needs a
paid Apple Developer Program membership ($99/year), which the project does not
have. The free path to iPhone/iPad is the PWA home-screen app (`../pwa/`):
Add to Home Screen in Safari, full-screen, no App Store, no fee — with the
honest limit that iOS can purge a home-screen web app's data if unused, so
`.weave` backups matter even more there.

## Project layout

```
desktop/
  dist/index.html                  copy of ../pocket-sheets.html (regenerate, never edit)
  src-tauri/
    Cargo.toml                     package + tauri-plugin-dialog + base64
    tauri.conf.json                productName, identifier, window, bundle targets, CSP, withGlobalTauri
    build.rs                       tauri_build::build()
    src/main.rs                    minimal shell + read/save commands
    capabilities/default.json      core:default + dialog:default
    icons/                         icon.png (1024 RGBA), icon.ico (Win), icon.icns (Mac)
  .github/workflows/build.yml      macOS (arm64 + Intel) + Windows builds, Release publish
  README.md                        this file
  SIZE-BUDGET.md                   the under-20-MB installer budget
```
