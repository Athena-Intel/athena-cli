use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient};

pub mod general;
pub use general::GeneralClient;
pub struct AgentsClient {
    pub http_client: HttpClient,
    pub general: GeneralClient,
}

impl AgentsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
            general: GeneralClient::new(config.clone())?,
        })
    }
}
