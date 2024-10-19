#[path = "spotify_api/spotify_token.rs"]
mod spotify_token;

use ::futures::future::join_all;
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

async fn get_albums_tracks(albums: Vec<Album>) -> Result<String, Errors> {
    let track_futures = albums
        .iter()
        .map(|album| get_album_data(album))
        .collect::<Vec<_>>();
    let album_tracks_list: Vec<_> = join_all(track_futures)
        .await
        .into_iter()
        .collect::<Result<Vec<_>, _>>()
        .expect("Error getting album tracks");

    let json_output = serde_json::to_string_pretty(&album_tracks_list)?;

    Ok(json_output)
}

async fn make_http_request(url: String) -> Result<reqwest::Response, Errors> {
    let token = SpotifyToken::new().await?;
    let client = reqwest::Client::new();

    let response = client
        .get(&url)
        .bearer_auth(token.get_access_token())
        .send()
        .await?;
    Ok(response)
}

async fn get_album_data(album: &Album) -> Result<Value, Errors> {
    let url = format!(
        "https://api.spotify.com/v1/albums/{}/tracks",
        album.get_id()
    );
    let response = make_http_request(url).await?;
    let json: Value = response.json().await?;
    let items = json["items"].as_array().ok_or(Errors::NoTracksFound)?;
    let track_names: Vec<String> = items
        .iter()
        .filter_map(|item| item["name"].as_str().map(|name| name.to_string()))
        .collect();
    let album_data = json!({
        "album_name": album.get_name(),
        "track_names": track_names
    });
    Ok(album_data)
}

async fn get_new_album_releases() -> Result<Vec<Album>, Errors> {
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
    let start = std::time::Instant::now();
    let albums = get_new_album_releases()
        .await
        .expect("Error getting new albums");
    let tracks = get_albums_tracks(albums)
        .await
        .expect("Error getting new albums tracks");

    println!("{}", tracks);
    println!("Time elapsed: {:?}", start.elapsed());
}
