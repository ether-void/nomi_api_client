//! # Nomi AI Client
//!
//! A Rust client library for interacting with the [Nomi.ai API](https://api.nomi.ai/docs/).
//!
//! ## Usage
//! ```no_run
//! use nomi_api_client::client::NomiClient;
//! use tokio;
//!
//! #[tokio::main]
//! async fn main() {
//!     let api_key = "your_api_key".to_string(); // Use environment variable don't expose api key
//!     let client = NomiClient::new(api_key);
//!
//!     match client.list_nomis().await {
//!         Ok(nomis) => {
//!             for nomi in nomis {
//!                 println!("Nomi: {} (UUID: {})", nomi.name, nomi.uuid);
//!             }
//!         }
//!         Err(e) => eprintln!("Error fetching Nomis: {}", e),
//!     }
//! }
//! ```
//!
pub mod client;
pub mod endpoints;
pub mod errors;
pub mod models;
