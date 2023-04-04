#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod parser;
use parser::parse_input;

fn main() {
    tauri::Builder::default()
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
