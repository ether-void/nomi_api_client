use nomi_ai_client::{client::NomiClient, errors::NomiError};
use std::env;

/// Ensure an API key is set before running tests.
fn get_api_key() -> String {
    env::var("NOMI_API_KEY").expect("NOMI_API_KEY must be set")
}

fn get_nomi_uuid() -> String {
    env::var("NOMI_UUID").expect("NOMI_UUID must be set")
}

#[tokio::test]
async fn test_list_nomis() {
    let client = NomiClient::new(get_api_key());
    let result = client.list_nomis().await;
    assert!(result.is_ok(), "Fetching Nomis failed");
    assert!(result.unwrap().len() > 0, "No Nomis found");
}

#[tokio::test]
async fn test_invalid_api_key() {
    let client = NomiClient::new("invalid_api_key".to_string());
    let result = client.list_nomis().await;

    match result {
        Err(NomiError::APIError(error_type)) => assert_eq!(error_type, "InvalidAPIKey"),
        _ => panic!("Expected InvalidAPIKey error"),
    }
}

#[tokio::test]
async fn test_get_nomi() {
    let client = NomiClient::new(get_api_key());
    let uuid = get_nomi_uuid();
    let result = client.get_nomi(&uuid).await;
    assert!(result.is_ok(), "Fetching Nomi failed");
}

#[tokio::test]
async fn test_get_avatar() {
    let client = NomiClient::new(get_api_key());
    let uuid = get_nomi_uuid();
    let result = client.get_avatar(&uuid).await;
    assert!(result.is_ok(), "Fetching avatar failed");
}

// #[tokio::test]
// async fn test_send_message() {
//     let client = NomiClient::new(get_api_key());
//     let uuid = get_nomi_uuid();
//     let message = "Hello!";
//     let result = client.send_message(&uuid, message).await;
//     assert!(result.is_ok(), "Sending message failed");
// }
