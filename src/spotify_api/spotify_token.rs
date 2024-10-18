use std::env;

use base64::encode;
use reqwest::Error;
use serde::Deserialize;
use dotenv::dotenv;


#[derive(Deserialize)]
pub struct SpotifyToken {
    access_token: String,
}

impl SpotifyToken {
    pub async fn new() -> Result<Self, Error> {
        dotenv().ok();
        let client_id = env::var("SPOTIFY_CLIENT_ID").expect("Client ID not found");
        let client_secret = env::var("SPOTIFY_CLIENT_SECRET").expect("Client Secret not found");
        let client = reqwest::Client::new();
        let params = [("grant_type", "client_credentials")];

        // Encode client_id and client_secret
        let credentials = format!("{}:{}", client_id, client_secret);
        let encoded_credentials = encode(credentials);

        let response = client
            .post("https://accounts.spotify.com/api/token")
            .header("Authorization", format!("Basic {}", encoded_credentials))
            .form(&params)
            .send()
            .await?;

        let token_response: SpotifyToken = response.json().await?;
        Ok(token_response)
    }

    pub fn get_access_token(&self) -> &str {
        &self.access_token
    }
}
