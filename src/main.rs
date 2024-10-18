#[path = "spotify_api/spotify_token.rs"]
mod spotify_token;

use dotenv::dotenv;
use reqwest;
use reqwest::Error;
use serde_json::Value;
use spotify_token::SpotifyToken;
use std::env; // Import the dotenv function

async fn get_new_album_releases() -> Result<(), Error> {
    dotenv().ok();
    let client_id = env::var("SPOTIFY_CLIENT_ID").expect("Client ID not found");
    let client_secret = env::var("SPOTIFY_CLIENT_SECRET").expect("Client Secret not found");
    let token = SpotifyToken::new(&client_id, &client_secret).await?;
    let url = "https://api.spotify.com/v1/browse/new-releases";
    // Create a client
    let client = reqwest::Client::new();

    let response = client
        .get(url)
        .bearer_auth(token.get_access_token())
        .send()
        .await?;

    // Parse the response body as JSON
    let json: Value = response.json().await?;

    // // Access the names of the albums
    let albums = &json["albums"]["items"];
    match albums.as_array() {
        Some(albums_array) => {
            for album in albums_array {
                match album["name"].as_str() {
                    Some(name) => println!("{}", name),
                    None => println!("No name found"),
                }
            }
        }
        None => {
            println!("No albums found");
            return Ok(());
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    get_new_album_releases()
        .await
        .expect("Failed to get new album releases");
}
