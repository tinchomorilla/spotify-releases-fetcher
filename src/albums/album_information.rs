pub struct Album {
    album_name: String,
    album_id: String,
}

impl Album {
    pub fn new(album_name: String, album_id: String) -> Self {
        Album {
            album_name,
            album_id,
        }
    }

    pub fn get_name(&self) -> &str {
        &self.album_name
    }

    pub fn get_id(&self) -> &str {
        &self.album_id
    }
}