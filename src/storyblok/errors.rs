use reqwest;
use serde_json;
use std::fmt;
use url;

#[derive(Debug)]
pub enum MyCustomError {
    HttpError,
    ParseError,
}
impl std::error::Error for MyCustomError {}

impl fmt::Display for MyCustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MyCustomError::HttpError => write!(f, "HTTP Error"),
            MyCustomError::ParseError => write!(f, "Parse Error"),
        }
    }
}

impl From<reqwest::Error> for MyCustomError {
    fn from(_: reqwest::Error) -> Self {
        MyCustomError::HttpError
    }
}

impl From<serde_json::Error> for MyCustomError {
    fn from(_: serde_json::Error) -> Self {
        MyCustomError::ParseError
    }
}

impl From<url::ParseError> for MyCustomError {
    fn from(_: url::ParseError) -> Self {
        MyCustomError::ParseError
    }
}
