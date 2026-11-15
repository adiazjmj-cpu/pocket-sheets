// Pocket Sheets — desktop shell (Tauri 2).
// The app itself is a single self-contained HTML file (dist/index.html); this
// binary only hosts it in the system webview. No plugins for data, no network,
// no filesystem access beyond what the webview itself provides (its own
// IndexedDB/localStorage and downloads for export) — except for the native
// save/read paths below. All of the app's behaviour lives in that one HTML file.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

/// Read a text file at an absolute path the user picked in the native
/// Open dialog. This is the desktop shell's native read path, and it exists
/// for one reason: the webview's `<input type="file">` picker does not
/// deliver the selected file to the app in Tauri on macOS — the dialog opens,
/// the user picks a file, and the input's `change` event never fires with
/// the file. Import (Open) therefore silently does nothing in the desktop
/// app. The live app file routes import through this command when it runs
/// inside Tauri (it checks `window.__TAURI__` first and falls back to the
/// file input in a plain browser).
#[tauri::command]
fn read_text_file(path: String) -> Result<String, String> {
    std::fs::read_to_string(&path).map_err(|e| e.to_string())
}

/// Read a binary file at an absolute path the user picked in the native
/// Open dialog, returned as base64 so it can cross the invoke bridge.
/// Covers .xlsx import (the only binary import Sheets has). The app decodes
/// it with `atob` on the JS side.
#[tauri::command]
fn read_bytes_file(path: String) -> Result<String, String> {
    use base64::Engine;
    let bytes = std::fs::read(&path).map_err(|e| e.to_string())?;
    Ok(base64::engine::general_purpose::STANDARD.encode(&bytes))
}

/// Write text to a file at an absolute path the user picked in the native
/// Save As dialog. This is the desktop shell's native save path, and it exists
/// for one reason: the webview's browser anchor download (`<a download>`) is
/// silently dropped by WebView2 on Windows, so export cannot use the anchor
/// in the desktop app. The live app file routes export through this command
/// when it runs inside Tauri (it checks `window.__TAURI__` first and falls
/// back to the anchor in a plain browser). Text exports (.csv, .weave) use
/// this command; binary exports (.xlsx) use save_bytes_file below.
#[tauri::command]
fn save_text_file(path: String, contents: String) -> Result<(), String> {
    std::fs::write(&path, contents).map_err(|e| e.to_string())
}

/// Write binary data to a file at an absolute path the user picked in the
/// native Save As dialog. The bytes arrive as base64 (the invoke bridge has
/// no binary type) and are decoded here. This is the binary counterpart of
/// save_text_file: .xlsx export goes through it in Tauri instead of falling
/// back to the anchor download.
#[tauri::command]
fn save_bytes_file(path: String, contents: String) -> Result<(), String> {
    use base64::Engine;
    let bytes = base64::engine::general_purpose::STANDARD
        .decode(contents)
        .map_err(|e| e.to_string())?;
    std::fs::write(&path, bytes).map_err(|e| e.to_string())
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![save_text_file, save_bytes_file, read_text_file, read_bytes_file])
        .run(tauri::generate_context!())
        .expect("error while running Pocket Sheets");
}
