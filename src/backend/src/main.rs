#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::{Manager, PhysicalSize};

mod parser;
use parser::parse_input;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let main_window = app.get_window("main").expect("Couldn't get the main window.");
            // main_window.set_min_size().expect("Couldn't change the windows dimensions.");
            main_window.set_size(PhysicalSize { height: 1600, width: 2400}).expect("Couldn't change the windows dimensions.");
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![parse_and_eval])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn parse_and_eval(input: &str) -> Result<String, String> {
    Ok(parse_input(input)
        .map_err(|e| format!("Parsing error: {}", e.to_string()))?
        .eval()
        .ok_or("Evaluation error: Invalid mathematical operation")?
        .to_string())
}
