use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Track {
    pub id: String,
    pub name: String,
    pub artists: Vec<String>,
    pub album: String,
    pub uri: String,
}

#[derive(Deserialize)]
pub struct SpotifyTrack {
    pub track: TrackInfo,
}

#[derive(Deserialize)]
pub struct TrackInfo {
    pub id: String,
    pub name: String,
    pub artists: Vec<Artist>,
    pub album: Album,
    pub uri: String,
}

#[derive(Deserialize)]
pub struct Artist {
    pub name: String,
}

#[derive(Deserialize)]
pub struct Album {
    pub name: String,
}

#[derive(Deserialize)]
pub struct LikedSongsResponse {
    pub items: Vec<SpotifyTrack>,
    pub next: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct TokenResponse {
    pub access_token: String,
    pub refresh_token: Option<String>,
    pub expires_in: u64,
}