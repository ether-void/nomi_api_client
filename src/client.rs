use crate::errors::NomiError;
use reqwest::{Client, Response};
use serde_json::Value;

pub struct NomiClient {
    api_key: String,
    client: Client,
}

impl NomiClient {
    pub fn new(api_key: impl Into<String>) -> Self {
        NomiClient {
            api_key: api_key.into(),
            client: Client::new(),
        }
    }

    /// Sends a GET request and handles errors.
    pub async fn get(&self, endpoint: &str) -> Result<Response, NomiError> {
        let url = format!("https://api.nomi.ai/v1/{}", endpoint);
        let response = self
            .client
            .get(&url)
            .header("Authorization", &self.api_key)
            .send()
            .await?;

        if response.status().is_success() {
            Ok(response)
        } else {
            Err(NomiError::from_response(response).await)
        }
    }

    /// Sends a POST request and handles errors.
    pub async fn post(&self, endpoint: &str, body: &Value) -> Result<Response, NomiError> {
        let url = format!("https://api.nomi.ai/v1/{}", endpoint);
        let response = self
            .client
            .post(&url)
            .header("Authorization", &self.api_key)
            .json(body)
            .send()
            .await?;

        if response.status().is_success() {
            Ok(response)
        } else {
            Err(NomiError::from_response(response).await)
        }
    }

    /// Gets a binary file and handles errors.
    /// Fetches the avatar of the Nomi identified by `id`.
    pub async fn get_binary(&self, endpoint: &str) -> Result<Vec<u8>, NomiError> {
        let url = format!("https://api.nomi.ai/v1/{}", endpoint);
        let response = self
            .client
            .get(&url)
            .header("Authorization", &self.api_key)
            .send()
            .await?;

        if response.status().is_success() {
            // Read the response body as bytes
            let bytes = response.bytes().await?;
            Ok(bytes.to_vec())
        } else {
            Err(NomiError::from_response(response).await)
        }
    }
}
