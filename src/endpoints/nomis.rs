use crate::{
    client::NomiClient,
    errors::NomiError,
    models::nomi::{Nomi, NomiListResponse},
};

impl NomiClient {
    /// Lists all Nomis associated with the API key.
    pub async fn list_nomis(&self) -> Result<Vec<Nomi>, NomiError> {
        let response = self.get("nomis").await?;
        let nomi_list: NomiListResponse = response.json().await?;
        Ok(nomi_list.nomis)
    }

    /// Fetches a specific Nomi by its UUID.
    pub async fn get_nomi(&self, uuid: &str) -> Result<Nomi, NomiError> {
        let response = self.get(&format!("nomis/{}", uuid)).await?;
        let nomi: Nomi = response.json().await?;
        Ok(nomi)
    }
}
