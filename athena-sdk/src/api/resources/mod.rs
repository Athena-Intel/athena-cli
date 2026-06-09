//! Service clients and API endpoints
//!
//! This module contains client implementations for:
//!
//! - **Agents**
//! - **Aop**
//! - **Assets**
//! - **Databases**
//! - **Users**
//! - **Query**
//! - **SemanticModel**
//! - **Threads**
//! - **Tools**

use crate::{ApiError, ClientConfig};

pub mod agents;
pub mod aop;
pub mod assets;
pub mod databases;
pub mod query;
pub mod semantic_model;
pub mod threads;
pub mod tools;
pub mod users;
pub struct ApiClient {
    pub config: ClientConfig,
    pub agents: AgentsClient,
    pub aop: AopClient,
    pub assets: AssetsClient,
    pub databases: DatabasesClient,
    pub users: UsersClient,
    pub query: QueryClient,
    pub semantic_model: SemanticModelClient,
    pub threads: ThreadsClient,
    pub tools: ToolsClient,
}

impl ApiClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            config: config.clone(),
            agents: AgentsClient::new(config.clone())?,
            aop: AopClient::new(config.clone())?,
            assets: AssetsClient::new(config.clone())?,
            databases: DatabasesClient::new(config.clone())?,
            users: UsersClient::new(config.clone())?,
            query: QueryClient::new(config.clone())?,
            semantic_model: SemanticModelClient::new(config.clone())?,
            threads: ThreadsClient::new(config.clone())?,
            tools: ToolsClient::new(config.clone())?,
        })
    }
}

pub use agents::AgentsClient;
pub use aop::AopClient;
pub use assets::AssetsClient;
pub use databases::DatabasesClient;
pub use query::QueryClient;
pub use semantic_model::SemanticModelClient;
pub use threads::ThreadsClient;
pub use tools::ToolsClient;
pub use users::UsersClient;
