use serde::{Deserialize, Serialize};

/// Request model for sending a chat message.
#[derive(Debug, Serialize)]
pub struct SendMessageRequest {
    #[serde(rename = "messageText")]
    pub message_text: String,
}

/// Response model for the chat message exchange.
#[derive(Debug, Deserialize)]
pub struct ChatResponse {
    #[serde(rename = "sentMessage")]
    pub sent_message: ChatMessage,
    #[serde(rename = "replyMessage")]
    pub reply_message: ChatMessage,
}

/// Represents an individual chat message.
#[derive(Debug, Deserialize)]
pub struct ChatMessage {
    pub uuid: String,
    pub text: String,
    pub sent: String,
}
