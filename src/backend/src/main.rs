#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

fn main() {
    tauri::Builder::default()
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
