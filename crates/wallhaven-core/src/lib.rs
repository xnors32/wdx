//! Core logic for Wallhaven Desktop.
//!
//! This crate intentionally does not depend on Dioxus. It provides domain models,
//! provider contracts, application services, storage helpers, and configuration.

pub mod config;
pub mod domain;
pub mod error;
pub mod providers;
pub mod services;
pub mod storage;

pub use error::CoreError;
pub use providers::wallhaven::normalize_search;
pub use providers::wallhaven::WallhavenClient;
pub use services::download::{run_download, DownloadRequest};

/// Convenience alias used by service and provider code.
pub type Result<T> = std::result::Result<T, CoreError>;
