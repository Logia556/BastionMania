use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct InstanceClient {
    pub client_public_key: String,
    pub client_address: String,
}