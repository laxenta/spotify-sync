// This declares our spotify module
pub mod spotify;

// Re-export our auth functions so they're easy to import
pub use spotify::auth::*;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            get_spotify_login_url  // Now we can use it directly!
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}