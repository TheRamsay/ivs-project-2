#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::{Manager, PhysicalSize};

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let main_window = app.get_window("main").expect("Couldn't get the main window.");
            main_window.set_size(PhysicalSize { height: 1100, width: 700 }).expect("Couldn't change the windows dimensions.");
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![hello])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

/// this is where we will write all the documentation
/// everything behind triple slash will be added
#[tauri::command]
fn hello(name: &str) -> Result<String, String> {
    if name.contains(' ') {
        Err("Name should not contain spaces".to_string())
    } else {
        Ok(format!("Hello, {}, 3 + 2 = {}", name, 42))
    }
}
