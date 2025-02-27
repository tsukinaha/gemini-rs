#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("serde: {0}")]
    Serde(#[from] serde_json::Error),
    #[error("http: {0}")]
    Http(#[from] reqwest::Error),
    #[error("gemini: {0:?}")]
    Gemini(crate::types::ErrorDetail),
}
