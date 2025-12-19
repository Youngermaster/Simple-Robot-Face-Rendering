//! # Teleoperation Monitoring System
//!
//! A Clean Architecture implementation for robotics teleoperation and monitoring.
//!
//! ## Architecture Layers
//!
//! 1. **Domain** - Core business logic (sensors, telemetry)
//! 2. **Application** - Use cases and services
//! 3. **Infrastructure** - External adapters (networking)
//! 4. **Presentation** - UI components
//!
//! ## Design Principles
//!
//! - Separation of Concerns
//! - Dependency Inversion
//! - Single Responsibility
//! - Testability

#![warn(clippy::all, rust_2018_idioms)]

// Core layers
pub mod domain;
pub mod application;
pub mod infrastructure;
pub mod presentation;

// Re-export commonly used types for convenience
pub use application::{AppConfig, TelemetryService};
pub use domain::telemetry::TelemetryData;
pub use infrastructure::Server;
pub use presentation::TeleoperationApp;
