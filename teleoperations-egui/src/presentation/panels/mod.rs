//! UI Panels
//!
//! Each panel is responsible for rendering one aspect of the telemetry data.

pub mod temperature;
pub mod imu;
pub mod occupancy;

// Re-export
pub use temperature::TemperaturePanel;
pub use imu::IMUPanel;
pub use occupancy::OccupancyPanel;
