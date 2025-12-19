//! Application Layer - Use Cases and Services
//!
//! This layer orchestrates the domain logic and coordinates between
//! the domain and infrastructure layers. It contains the business workflows.

pub mod config;
pub mod telemetry_service;

// Re-export
pub use config::AppConfig;
pub use telemetry_service::TelemetryService;
