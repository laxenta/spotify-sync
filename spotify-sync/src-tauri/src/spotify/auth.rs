use serde::{Deserialize, Serialize};
use dotenv::dotenv;
use std::env;

#[derive(Debug)]
pub struct SpotifyAuth {
    pub client_id: String,
    pub client_secret: String,
    pub redirect_uri: String,
}

//hold spotify tokens
#[derive(Debug, Serialize, Deserialize)]
pub struct TokenResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: i64,
    pub refresh_token: Option<String>,
    pub scope: String,
}

impl SpotifyAuth {
    pub fn new(client_id: String, client_secret: String, redirect_uri: String) -> Self {
        Self {
            client_id,
            client_secret,
            redirect_uri,
        }
    }

    pub fn get_authorize_url(&self) -> String {
        let base_url = "https://accounts.spotify.com/authorize";

        let params = vec![
            ("client_id", self.client_id.as_str()),
            ("response_type", "code"),
            ("redirect_uri", self.redirect_uri.as_str()),
            ("scope", "playlist-read-private playlist-read-collaborative user-library-read"),
            ("state", "some_random_state_here"), //AAAAAA make it rand later
        ];
        self.build_url(base_url, &params)
    }

    fn build_url(&self, base: &str, params: &[(&str, &str)]) -> String {
        let mut url = base.to_string() + "?";

        for (i, (key, value)) in params.iter().enumerate() {
            if i > 0 {
                url.push('&');
            }
            url.push_str(&format!("{}={}", key, value));
        }
        url
    }
}

// Tauri command to get the login URL - this will be callable from React
#[tauri::command]
pub fn get_spotify_login_url() -> Result<String, String> {
    // Load environment variables from .env file
    dotenv().ok();
    
    let client_id = env::var("SPOTIFY_CLIENT_ID")
        .map_err(|_| "SPOTIFY_CLIENT_ID not found in .env file".to_string())?;
        
    let client_secret = env::var("SPOTIFY_CLIENT_SECRET")
        .map_err(|_| "SPOTIFY_CLIENT_SECRET not found in .env file".to_string())?;
        
    let redirect_uri = env::var("SPOTIFY_REDIRECT_URI")
        .map_err(|_| "SPOTIFY_REDIRECT_URI not found in .env file".to_string())?;

    let auth = SpotifyAuth::new(client_id, client_secret, redirect_uri);
    
    let url = auth.get_authorize_url();
    println!("Generated Spotify login URL: {}", url); // This will show in terminal
    
    Ok(url)
}