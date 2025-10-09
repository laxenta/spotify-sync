use reqwest::Client;
use crate::spotify::auth::SpotifyAuth;
use crate::spotify::types::{Track, LikedSongsResponse};

pub struct SpotifyApi {
    auth: SpotifyAuth,
    client: Client,
}

impl SpotifyApi {
    pub fn new(auth: SpotifyAuth) -> Self {
        SpotifyApi {
            auth,
            client: Client::new(),
        }
    }

    pub async fn get_liked_songs(&self) -> Result<Vec<Track>, Box<dyn std::error::Error>> {
        let mut tracks = Vec::new();
        let mut offset = 0;
        let limit = 50;

        loop {
            let url = format!(
                "https://api.spotify.com/v1/me/tracks?limit={}&offset={}",
                limit, offset
            );

            let response = self
                .client
                .get(&url)
                .bearer_auth(&self.auth.access_token)
                .send()
                .await?;

            let data: LikedSongsResponse = response.json().await?;

            for item in data.items {
                tracks.push(Track {
                    id: item.track.id,
                    name: item.track.name,
                    artists: item.track.artists.iter().map(|a| a.name.clone()).collect(),
                    album: item.track.album.name,
                    uri: item.track.uri,
                });
            }

            if data.next.is_none() {
                break;
            }

            offset += limit;
        }

        Ok(tracks)
    }

    pub async fn add_to_liked(&self, tracks: Vec<Track>) -> Result<(), Box<dyn std::error::Error>> {
        for chunk in tracks.chunks(50) {
            let ids: Vec<String> = chunk
                .iter()
                .map(|t| t.id.clone())
                .collect();

            let url = format!(
                "https://api.spotify.com/v1/me/tracks?ids={}",
                ids.join(",")
            );

            self
                .client
                .put(&url)
                .bearer_auth(&self.auth.access_token)
                .send()
                .await?;
        }

        Ok(())
    }
}