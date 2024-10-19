use base64::encode;
use dotenv::dotenv;
use once_cell::sync::OnceCell;
use serde::Deserialize;
use serde_json::from_str;
use std::env;
use ureq;

#[derive(Deserialize)]
pub struct SpotifyToken {
    access_token: String,
}

impl SpotifyToken {
    fn new() -> Result<Self, Box<dyn std::error::Error>> {
        dotenv().ok();
        let client_id = env::var("SPOTIFY_CLIENT_ID")?;
        let client_secret = env::var("SPOTIFY_CLIENT_SECRET")?;
        let params = [("grant_type", "client_credentials")];

        // Encode client_id and client_secret
        let credentials = format!("{}:{}", client_id, client_secret);
        let encoded_credentials = encode(credentials);

        let response = ureq::post("https://accounts.spotify.com/api/token")
            .set("Authorization", &format!("Basic {}", encoded_credentials))
            .send_form(&params)?;
        let response_body = response.into_string()?;
        let token_response: SpotifyToken = from_str(&response_body)?;
        Ok(token_response)
    }

    pub fn get_access_token(&self) -> &str {
        &self.access_token
    }
}

lazy_static::lazy_static! {
    static ref TOKEN: OnceCell<SpotifyToken> = OnceCell::new();
}

pub fn initialize_token() -> Result<(), Box<dyn std::error::Error>> {
    TOKEN.get_or_init(|| {
        SpotifyToken::new().unwrap_or_else(|err| {
            panic!("Failed to initialize SpotifyToken: {}", err);
        })
    });
    Ok(())
}

pub fn get_token() -> &'static SpotifyToken {
    TOKEN.get().expect("Token not initialized")
}
