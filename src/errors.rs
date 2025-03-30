use reqwest::StatusCode;
use serde::Deserialize;
use serde_json::Error as SerdeError;
use thiserror::Error;

/// Represents an API error response.
#[derive(Debug, Deserialize)]
pub struct APIErrorResponse {
    pub error: APIError,
}

/// Describes the API error type.
#[derive(Debug, Deserialize)]
pub struct APIError {
    #[serde(rename = "type")]
    pub error_type: String,
}

/// Represents different error cases for the Nomi AI client.
#[derive(Debug, Error)]
pub enum NomiError {
    /// HTTP request error (e.g., network failure).
    #[error("HTTP request failed: {0}")]
    ReqwestError(#[from] reqwest::Error),

    /// API returned an error response.
    #[error("API error: {0}")]
    APIError(String),

    /// JSON deserialization error.
    #[error("Failed to parse response: {0}")]
    JSONError(#[from] SerdeError),

    /// Unexpected response format.
    #[error("Unexpected response format")]
    UnexpectedFormat,
}

impl NomiError {
    /// Attempts to parse an API error from a response.
    pub async fn from_response(response: reqwest::Response) -> Self {
        match response.status() {
            StatusCode::UNAUTHORIZED
            | StatusCode::FORBIDDEN
            | StatusCode::NOT_FOUND
            | StatusCode::BAD_REQUEST => {
                if let Ok(api_error) = response.json::<APIErrorResponse>().await {
                    NomiError::APIError(api_error.error.error_type)
                } else {
                    NomiError::UnexpectedFormat
                }
            }
            _ => NomiError::ReqwestError(response.error_for_status().unwrap_err()),
        }
    }
}
