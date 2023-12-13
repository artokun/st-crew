use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ApiError {
    pub error: String,
    pub message: String,
}
