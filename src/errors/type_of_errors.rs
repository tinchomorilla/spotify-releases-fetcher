use std::fmt;
use std::error::Error as StdError;


#[derive(Debug)]
pub enum Errors {
    NoAlbumsFound,
    ReqwestError(reqwest::Error),
    NoTracksFound,
    SerdeJsonError(serde_json::Error),
    Other(String),
}


impl fmt::Display for Errors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Errors::NoAlbumsFound => write!(f, "No albums found"),
            Errors::ReqwestError(e) => write!(f, "Request error: {}", e),
            Errors::NoTracksFound => write!(f, "No tracks found"),
            Errors::SerdeJsonError(e) => write!(f, "Serde JSON error: {}", e),
            Errors::Other(e) => write!(f, "Other error: {}", e),
        }
    }
}

impl std::error::Error for Errors {}

impl From<reqwest::Error> for Errors {
    fn from(error: reqwest::Error) -> Self {
        Errors::ReqwestError(error)
    }
}

impl From<Box<dyn StdError>> for Errors {
    fn from(error: Box<dyn StdError>) -> Self {
        Errors::Other(error.to_string())
    }
}

impl From<serde_json::Error> for Errors {
    fn from(error: serde_json::Error) -> Self {
        Errors::SerdeJsonError(error)
    }
}