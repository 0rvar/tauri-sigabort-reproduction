#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::{AppHandle, Manager};

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![crash])
        .run(tauri::generate_context!())
        .unwrap();
}

#[tauri::command]
async fn crash(handle: AppHandle) -> Result<(), String> {
    for _ in 0..10_000 {
        handle.emit_all("waddup", ()).unwrap();
    }
    Ok(())
}
