//! Presentation Layer - UI Components
//!
//! This layer contains all UI-related code organized into logical components.
//! Each panel is self-contained and focuses on displaying one aspect of the data.

pub mod app;
pub mod panels;
pub mod view_models;

// Re-export
pub use app::TeleoperationApp;
