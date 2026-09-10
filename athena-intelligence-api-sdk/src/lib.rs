//! # Athena Intelligence API SDK
//!
//! The official Rust SDK for the Athena Intelligence API.
//!
//! ## Getting Started
//!
//! ```rust
//! use athena_intelligence_api_sdk::prelude::*;
//!
//! #[tokio::main]
//! async fn main() {
//!     let config = ClientConfig {
//!         api_key: Some("<value>".to_string()),
//!         ..Default::default()
//!     };
//!     let client = AthenaIntelligenceApiClient::new(config).expect("Failed to build client");
//!     client
//!         .aop
//!         .create(
//!             &AopCreateRequestIn {
//!                 agent_id: Some("research_agent".to_string()),
//!                 description: Some("Generates market research reports".to_string()),
//!                 icon: Some("ListTodo".to_string()),
//!                 parent_folder_id: Some("asset_folder_12345".to_string()),
//!                 prompt: Some(
//!                     "Generate a comprehensive market research report for [[ company ]]".to_string(),
//!                 ),
//!                 section: Some("Research".to_string()),
//!                 title: Some("Market Research Report".to_string()),
//!                 workspace_id: Some("workspace_abc123".to_string()),
//!                 ..Default::default()
//!             },
//!             None,
//!         )
//!         .await;
//! }
//! ```
//!
//! ## Modules
//!
//! - [`api`] - Core API types and models
//! - [`client`] - Client implementations
//! - [`config`] - Configuration options
//! - [`core`] - Core utilities and infrastructure
//! - [`error`] - Error types and handling
//! - [`prelude`] - Common imports for convenience

pub mod api;
pub mod client;
pub mod config;
pub mod core;
pub mod environment;
pub mod error;
pub mod prelude;

pub use client::*;
pub use config::*;
pub use core::*;
pub use environment::*;
pub use error::{ApiError, BuildError};
