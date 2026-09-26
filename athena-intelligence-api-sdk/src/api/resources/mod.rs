//! Service clients and API endpoints
//!
//! This module contains client implementations for:
//!
//! - **Aop**
//! - **Approvals**
//! - **Assets**
//! - **Automations**
//! - **CollabAgents**
//! - **Computer**
//! - **Databases**
//! - **Events**
//! - **System**
//! - **Users**
//! - **Meetings**
//! - **Query**
//! - **Scripts**
//! - **SemanticModel**
//! - **Sessions**
//! - **Threads**
//! - **Toolkits**
//! - **Tools**
//! - **Presentation**
//! - **Workspaces**
//! - **Agents**

use crate::{ApiError, ClientConfig};

pub mod agents;
pub mod aop;
pub mod approvals;
pub mod assets;
pub mod automations;
pub mod collab_agents;
pub mod computer;
pub mod databases;
pub mod events;
pub mod meetings;
pub mod presentation;
pub mod query;
pub mod scripts;
pub mod semantic_model;
pub mod sessions;
pub mod system;
pub mod threads;
pub mod toolkits;
pub mod tools;
pub mod users;
pub mod workspaces;
pub struct ApiClient {
    pub config: ClientConfig,
    pub aop: AopClient,
    pub approvals: ApprovalsClient,
    pub assets: AssetsClient,
    pub automations: AutomationsClient,
    pub collab_agents: CollabAgentsClient,
    pub computer: ComputerClient,
    pub databases: DatabasesClient,
    pub events: EventsClient,
    pub system: SystemClient,
    pub users: UsersClient,
    pub meetings: MeetingsClient,
    pub query: QueryClient,
    pub scripts: ScriptsClient,
    pub semantic_model: SemanticModelClient,
    pub sessions: SessionsClient,
    pub threads: ThreadsClient,
    pub toolkits: ToolkitsClient,
    pub tools: ToolsClient,
    pub presentation: PresentationClient,
    pub workspaces: WorkspacesClient,
    pub agents: AgentsClient,
}

impl ApiClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            config: config.clone(),
            aop: AopClient::new(config.clone())?,
            approvals: ApprovalsClient::new(config.clone())?,
            assets: AssetsClient::new(config.clone())?,
            automations: AutomationsClient::new(config.clone())?,
            collab_agents: CollabAgentsClient::new(config.clone())?,
            computer: ComputerClient::new(config.clone())?,
            databases: DatabasesClient::new(config.clone())?,
            events: EventsClient::new(config.clone())?,
            system: SystemClient::new(config.clone())?,
            users: UsersClient::new(config.clone())?,
            meetings: MeetingsClient::new(config.clone())?,
            query: QueryClient::new(config.clone())?,
            scripts: ScriptsClient::new(config.clone())?,
            semantic_model: SemanticModelClient::new(config.clone())?,
            sessions: SessionsClient::new(config.clone())?,
            threads: ThreadsClient::new(config.clone())?,
            toolkits: ToolkitsClient::new(config.clone())?,
            tools: ToolsClient::new(config.clone())?,
            presentation: PresentationClient::new(config.clone())?,
            workspaces: WorkspacesClient::new(config.clone())?,
            agents: AgentsClient::new(config.clone())?,
        })
    }
}

pub use agents::AgentsClient;
pub use aop::AopClient;
pub use approvals::ApprovalsClient;
pub use assets::AssetsClient;
pub use automations::AutomationsClient;
pub use collab_agents::CollabAgentsClient;
pub use computer::ComputerClient;
pub use databases::DatabasesClient;
pub use events::EventsClient;
pub use meetings::MeetingsClient;
pub use presentation::PresentationClient;
pub use query::QueryClient;
pub use scripts::ScriptsClient;
pub use semantic_model::SemanticModelClient;
pub use sessions::SessionsClient;
pub use system::SystemClient;
pub use threads::ThreadsClient;
pub use toolkits::ToolkitsClient;
pub use tools::ToolsClient;
pub use users::UsersClient;
pub use workspaces::WorkspacesClient;
