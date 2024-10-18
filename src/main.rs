#[path = "spotify_api/spotify_token.rs"]
mod spotify_token;
use spotify_token::SpotifyToken;

#[path = "errors/type_of_errors.rs"]
mod type_of_errors;
use type_of_errors::Errors;

#[path = "albums/album_information.rs"]
mod album_information;
use album_information::Album;

use reqwest;
use serde_json::json;
use serde_json::Value;

async fn get_new_album_tracks() -> Result<(), Errors> {
    let albums = get_new_album_releases_ids().await?;
    let token = SpotifyToken::new().await?;
    let client = reqwest::Client::new();

    let mut album_tracks_list = Vec::new();

    for album in albums {
        let url = format!(
            "https://api.spotify.com/v1/albums/{}/tracks",
            album.get_id()
        );
        let response = client
            .get(&url)
            .bearer_auth(token.get_access_token())
            .send()
            .await?;

        let json: Value = response.json().await?;
        let items = json["items"].as_array().ok_or(Errors::NoTracksFound)?;

        let track_names: Vec<String> = items
            .iter()
            .filter_map(|item| item["name"].as_str().map(|name| name.to_string()))
            .collect();

        // Create a JSON object with the desired fields
        let album_data = json!({
            "album_name": album.get_name(),
            "track_names": track_names
        });

        album_tracks_list.push(album_data);
    }

    let json_output = serde_json::to_string_pretty(&album_tracks_list)?;
    println!("{}", json_output);

    Ok(())
}

async fn get_new_album_releases_ids() -> Result<Vec<Album>, Errors> {
    let token = SpotifyToken::new().await?;
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

    // Access to the album fields
    let albums = &json["albums"]["items"];
    let albums_array = albums.as_array().ok_or(Errors::NoAlbumsFound)?;
    let mut albums = Vec::new();

    for album in albums_array {
        match (album["id"].as_str(), album["name"].as_str()) {
            (Some(id), Some(name)) => {
                let album = Album::new(name.to_string(), id.to_string());
                albums.push(album);
            }
            _ => println!("No ID or name found"),
        }
    }

    Ok(albums)
}

#[tokio::main]
async fn main() {
    get_new_album_tracks()
        .await
        .expect("Error getting new album tracks");
}
