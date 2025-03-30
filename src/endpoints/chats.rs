use crate:: {
    client::NomiClient,
    errors::NomiError,
    models::chat::{ChatResponse, SendMessageRequest},
};

use serde_json::json;

impl NomiClient {
    /// Sends a message to main chat and get reply
    pub async fn send_message(&self, uuid: &str, message_text: &str) -> Result<ChatResponse, NomiError> {
        let url = format!("nomis/{}/chat", uuid);
        let body = SendMessageRequest {
            message_text: message_text.to_string(),
        };

        let response = self.post(&url, &json!(body)).await?;

        if response.status().is_success() {
            let chat_response: ChatResponse = response.json().await?;
            Ok(chat_response)
        } else {
            Err(NomiError::from_response(response).await)
        }
    }
}
