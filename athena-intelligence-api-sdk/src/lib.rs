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
//!         .get_batch_status(
//!             &"batch_0f4c3c3e-1c1a-4a3e-9a5f-0d3f1a2b3c4d".to_string(),
//!             &GetBatchStatusQueryRequest {
//!                 status: Some("terminal".to_string()),
//!                 limit: Some(200),
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
