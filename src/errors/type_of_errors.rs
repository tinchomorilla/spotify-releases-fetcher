use std::fmt;

#[derive(Debug)]
pub enum Errors {
    NoAlbumsFound,
    ReqwestError(reqwest::Error),
}

impl fmt::Display for Errors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Errors::NoAlbumsFound => write!(f, "No albums found"),
            Errors::ReqwestError(e) => write!(f, "Request error: {}", e),
        }
    }
}

impl From<reqwest::Error> for Errors {
    fn from(error: reqwest::Error) -> Self {
        Errors::ReqwestError(error)
    }
}