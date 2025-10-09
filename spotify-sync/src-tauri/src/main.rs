#![cfg_attr(all(not(debug_assertions), target_os = "windows"), windows_subsystem = "windows")]

use tauri::State;
use std::sync::Mutex;
use std::collections::HashMap;

mod spotify;
mod storage;

use spotify::auth::SpotifyAuth;
use spotify::api::SpotifyApi;
use spotify::types::Track;
use storage::LocalStorage;

struct AppState {
    from_auth: Mutex<Option<SpotifyAuth>>,
    to_auth: Mutex<Option<SpotifyAuth>>,
    from_tracks: Mutex<Vec<Track>>,
    to_tracks: Mutex<Vec<Track>>,
    storage: LocalStorage,
}

#[tauri::command]
async fn get_oauth_url(panel: String) -> Result<String, String> {
    SpotifyAuth::get_auth_url(&panel).map_err(|e| e.to_string())
}

#[tauri::command]
async fn handle_callback(
    panel: String,
    code: String,
    state: State<'_, AppState>,
) -> Result<String, String> {
    let mut auth = SpotifyAuth::new();
    auth.exchange_code(&code).await
        .map_err(|e| e.to_string())?;
    
    state.storage.save_token(&panel, &auth.access_token)
        .map_err(|e| e.to_string())?;
    
    if panel == "from" {
        *state.from_auth.lock().unwrap() = Some(auth);
    } else {
        *state.to_auth.lock().unwrap() = Some(auth);
    }
    
    Ok("Successfully authenticated!".to_string())
}

#[tauri::command]
async fn fetch_liked_songs(panel: String, state: State<'_, AppState>) -> Result<Vec<Track>, String> {
    let auth_option = if panel == "from" {
        state.from_auth.lock().unwrap().clone()
    } else {
        state.to_auth.lock().unwrap().clone()
    };
    
    let auth = auth_option.ok_or("Not authenticated")?;
    let api = SpotifyApi::new(auth);
    
    let tracks = api.get_liked_songs().await
        .map_err(|e| e.to_string())?;
    
    if panel == "from" {
        *state.from_tracks.lock().unwrap() = tracks.clone();
    } else {
        *state.to_tracks.lock().unwrap() = tracks.clone();
    }
    
    Ok(tracks)
}

#[tauri::command]
async fn transfer_tracks(state: State<'_, AppState>) -> Result<String, String> {
    let to_auth = state.to_auth.lock().unwrap().clone()
        .ok_or("Target account not authenticated")?;
    
    let tracks = state.from_tracks.lock().unwrap().clone();
    
    let api = SpotifyApi::new(to_auth);
    api.add_to_liked(tracks).await
        .map_err(|e| e.to_string())?;
    
    Ok("Transfer complete!".to_string())
}

#[tauri::command]
async fn load_saved_tokens(state: State<'_, AppState>) -> Result<HashMap<String, bool>, String> {
    let mut result = HashMap::new();
    
    if let Ok(token) = state.storage.load_token("from") {
        if !token.is_empty() {
            result.insert("from".to_string(), true);
        }
    }
    
    if let Ok(token) = state.storage.load_token("to") {
        if !token.is_empty() {
            result.insert("to".to_string(), true);
        }
    }
    
    Ok(result)
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(AppState {
            from_auth: Mutex::new(None),
            to_auth: Mutex::new(None),
            from_tracks: Mutex::new(Vec::new()),
            to_tracks: Mutex::new(Vec::new()),
            storage: LocalStorage::new(),
        })
        .invoke_handler(tauri::generate_handler![
            get_oauth_url,
            handle_callback,
            fetch_liked_songs,
            transfer_tracks,
            load_saved_tokens,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
