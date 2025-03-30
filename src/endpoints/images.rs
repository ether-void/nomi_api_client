use crate::{
    client::NomiClient,
    errors::NomiError
};

impl NomiClient {
    /// Fetches the avatar image for a specific Nomi by its UUID.
    pub async fn get_avatar(&self, uuid: &str) -> Result<Vec<u8>, NomiError> {
        let response = self.get_binary(&format!("nomis/{}/avatar", uuid)).await?;
        Ok(response)
    }
}
