use serde::{Deserialize, Serialize};

/// Represents a successful Nomi entity.
#[derive(Debug, Serialize, Deserialize)]
pub struct Nomi {
    pub uuid: String,
    pub gender: String,
    pub name: String,
    pub created: String,
    #[serde(rename = "relationshipType")]
    pub relationship_type: String,
}

/// Represents a list of Nomis.
#[derive(Debug, Deserialize)]
pub struct NomiListResponse {
    pub nomis: Vec<Nomi>,
}
