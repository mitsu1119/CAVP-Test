use std::io;

#[derive(Debug)]
pub enum CavpError {
    IoError(io::Error),
    ReqwestError(reqwest::Error),
    TestFailed(String),
}

impl From<io::Error> for CavpError {
    fn from(value: io::Error) -> Self {
        Self::IoError(value)
    }
}

impl From<reqwest::Error> for CavpError {
    fn from(value: reqwest::Error) -> Self {
        Self::ReqwestError(value)
    }
}
