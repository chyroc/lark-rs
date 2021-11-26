use std::convert::From;

use reqwest::Error as ReqwestError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Error {
    pub code: i32,
    pub message: String,
}

impl Error {
    pub fn new(code: i32, message: String) -> Error {
        Error { code, message }
    }
}

impl From<ReqwestError> for Error {
    fn from(e: ReqwestError) -> Self {
        Error {
            code: -1,
            message: e.to_string(),
        }
    }
}