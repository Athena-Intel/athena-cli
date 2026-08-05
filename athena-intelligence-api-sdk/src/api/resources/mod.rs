//! Service clients and API endpoints
//!
//! This module contains client implementations for:
//!
//! - **Agents**
//! - **Aop**
//! - **Assets**
//! - **CollabAgents**
//! - **Computer**
//! - **Databases**
//! - **Users**
//! - **Meetings**
//! - **Query**
//! - **SemanticModel**
//! - **Sessions**
//! - **Threads**
//! - **Toolkits**
//! - **Tools**
//! - **Workspaces**

use crate::{ApiError, ClientConfig};

pub mod agents;
pub mod aop;
pub mod assets;
pub mod collab_agents;
pub mod computer;
pub mod databases;
pub mod meetings;
pub mod query;
pub mod semantic_model;
pub mod sessions;
pub mod threads;
pub mod toolkits;
pub mod tools;
pub mod users;
pub mod workspaces;
pub struct ApiClient {
    pub config: ClientConfig,
    pub agents: AgentsClient,
    pub aop: AopClient,
    pub assets: AssetsClient,
    pub collab_agents: CollabAgentsClient,
    pub computer: ComputerClient,
    pub databases: DatabasesClient,
    pub users: UsersClient,
    pub meetings: MeetingsClient,
    pub query: QueryClient,
    pub semantic_model: SemanticModelClient,
    pub sessions: SessionsClient,
    pub threads: ThreadsClient,
    pub toolkits: ToolkitsClient,
    pub tools: ToolsClient,
    pub workspaces: WorkspacesClient,
}

impl ApiClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            config: config.clone(),
            agents: AgentsClient::new(config.clone())?,
            aop: AopClient::new(config.clone())?,
            assets: AssetsClient::new(config.clone())?,
            collab_agents: CollabAgentsClient::new(config.clone())?,
            computer: ComputerClient::new(config.clone())?,
            databases: DatabasesClient::new(config.clone())?,
            users: UsersClient::new(config.clone())?,
            meetings: MeetingsClient::new(config.clone())?,
            query: QueryClient::new(config.clone())?,
            semantic_model: SemanticModelClient::new(config.clone())?,
            sessions: SessionsClient::new(config.clone())?,
            threads: ThreadsClient::new(config.clone())?,
            toolkits: ToolkitsClient::new(config.clone())?,
            tools: ToolsClient::new(config.clone())?,
            workspaces: WorkspacesClient::new(config.clone())?,
        })
    }
}

pub use agents::AgentsClient;
pub use aop::AopClient;
pub use assets::AssetsClient;
pub use collab_agents::CollabAgentsClient;
pub use computer::ComputerClient;
pub use databases::DatabasesClient;
pub use meetings::MeetingsClient;
pub use query::QueryClient;
pub use semantic_model::SemanticModelClient;
pub use sessions::SessionsClient;
pub use threads::ThreadsClient;
pub use toolkits::ToolkitsClient;
pub use tools::ToolsClient;
pub use users::UsersClient;
pub use workspaces::WorkspacesClient;
