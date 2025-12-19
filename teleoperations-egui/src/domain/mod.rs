//! Domain Layer - Core Business Logic
//!
//! This layer contains the core business entities and rules.
//! It has NO dependencies on external frameworks or libraries.
//! This makes it easy to test and reuse across different contexts.

pub mod sensor;
pub mod telemetry;

// Re-export commonly used types
pub use sensor::{Sensor, SensorReading, SensorType};
pub use telemetry::{TelemetryData, TelemetryEvent};
