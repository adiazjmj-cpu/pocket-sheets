# Pocket Sheets — Size Budget (standing rule)

_Mirrored from the Pocket Notes size budget (2026-11-13). Same rule, same
reasoning: the professional alternatives are 100–600 MB because they bundle
a runtime; Pocket Sheets does not bundle one._

## The comparison (verified 2026-11-13, for the suite)

| App | Download / install size | Why |
|---|---|---|
| OneNote, Mac (Mac App Store) | **626 MB** | Native Office app, large binary + resources |
| Logseq, Windows | **~135 MB** | Electron — bundles a whole Chromium |
| Obsidian, desktop | **~124 MB** | Electron — bundles a whole Chromium |
| Pocket Sheets, desktop (Tauri, target) | **under 20 MB** | Tauri uses the system webview; the app itself is one ~116 KB HTML file |

## The budget

- **Each desktop installer (Mac .dmg, Windows .msi/.exe): under 20 MB.** A
  generous ceiling, not a tight one — the app file is ~116 KB and the Tauri
  shell is a few MB. If a build exceeds it, the build must not ship.
- **The app file (pocket-sheets.html): under 500 KB.** It is 118,871 bytes
  at v1.1. This leaves room to grow (charts, queries, linked records)
  without ballooning.
- **The web app (PWA) total: under 1 MB** (HTML + manifest + icons + sw.js).

## Guardrails (set in place, not just listed)

1. **Tauri, not Electron — standing rule.** Tauri uses the system webview
   (WebKit on Mac, WebView2 on Windows). Electron bundles a whole Chromium
   (~100 MB+) and would blow the budget by itself. Never switch to Electron.
2. **No bundled runtime, no bundled assets.** Single HTML file, inline SVG
   icons, system fonts. No web fonts, no bundled images, no bundled media.
3. **Release binary optimized for size.** `src-tauri/Cargo.toml`
   `[profile.release]`: `opt-level = "s"`, `strip = true`, `lto = true`,
   `codegen-units = 1`, `panic = "abort"`.
4. **Audit every dependency.** Every new Rust crate or JS library must be
   justified; no dependency for a trivial function. Today: zero JS
   dependencies (pure JS) and minimal Rust deps (Tauri + dialog plugin +
   base64). Keep it that way. (Sheets deliberately omits the HTTP client
   Pocket Notes carries — it has no web-clip fetch — to stay lean.)
5. **Measure on every build — enforced in CI.**
   `.github/workflows/build.yml` has a "Check installer size (20 MB budget)"
   step after the build: it lists the installer sizes and **fails the job**
   if any installer exceeds 20 MB. If it fails, find what ballooned it and
   shrink it — do not raise the budget.
6. **User data is separate from app size.** Workbooks live in IndexedDB
   (user data), not in the app bundle. The app stays small no matter how
   many workbooks the user has.
7. **No feature justifies bloat.** If a feature would balloon the size
   (e.g., bundling a large library), find the smaller way or do not ship it.
