use reqwest::Error;
use serde::Deserialize;
use base64::encode;

#[derive(Deserialize)]
pub struct SpotifyToken {
    access_token: String,
}

impl SpotifyToken {
    pub async fn new(client_id: &str, client_secret: &str) -> Result<Self, Error> {
        let client = reqwest::Client::new();
        let params = [
            ("grant_type", "client_credentials"),
        ];

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