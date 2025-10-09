use reqwest::Client;
use std::env;
use crate::spotify::types::TokenResponse;

#[derive(Clone, Debug)]
pub struct SpotifyAuth {
    pub access_token: String,
    pub refresh_token: Option<String>,
}

impl SpotifyAuth {
    pub fn new() -> Self {
        SpotifyAuth {
            access_token: String::new(),
            refresh_token: None,
        }
    }

    pub fn get_auth_url(panel: &str) -> Result<String, Box<dyn std::error::Error>> {
        let client_id = env::var("SPOTIFY_CLIENT_ID")?;
        let redirect_uri = env::var("SPOTIFY_REDIRECT_URI")?;

        let scopes = "user-library-read user-library-modify";
        
        let url = format!(
            "https://accounts.spotify.com/authorize?\
            client_id={}&\
            response_type=code&\
            redirect_uri={}&\
            scope={}&\
            state={}",
            client_id, redirect_uri, scopes, panel
        );

        Ok(url)
    }

    pub async fn exchange_code(&mut self, code: &str) -> Result<(), Box<dyn std::error::Error>> {
        let client_id = env::var("SPOTIFY_CLIENT_ID")?;
        let client_secret = env::var("SPOTIFY_CLIENT_SECRET")?;
        let redirect_uri = env::var("SPOTIFY_REDIRECT_URI")?;

        let client = Client::new();
        let response = client
            .post("https://accounts.spotify.com/api/token")
            .form(&[
                ("grant_type", "authorization_code"),
                ("code", code),
                ("redirect_uri", &redirect_uri),
                ("client_id", &client_id),
                ("client_secret", &client_secret),
            ])
            .send()
            .await?;

        let token_response: TokenResponse = response.json().await?;
        self.access_token = token_response.access_token;
        self.refresh_token = token_response.refresh_token;

        Ok(())
    }
}