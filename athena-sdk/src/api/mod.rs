//! API client and types for the Athena Intelligence API
//!
//! This module contains all the API definitions including request/response types
//! and client implementations for interacting with the API.
//!
//! ## Modules
//!
//! - [`resources`] - Service clients and endpoints

pub mod resources;

pub use resources::{
    AgentsClient, AopClient, ApiClient, AssetsClient, DatabasesClient, QueryClient,
    SemanticModelClient, ThreadsClient, ToolsClient,
};

pub use athena_types::*;
