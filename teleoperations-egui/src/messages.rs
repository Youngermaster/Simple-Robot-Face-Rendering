//! Message types for sensor communication
//!
//! Defines the data structures exchanged between Python sensors and the Rust backend.

use serde::{Deserialize, Serialize};

/// All possible sensor messages
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum SensorMessage {
    /// Temperature sensor reading
    Temperature {
        value: f32,
        unit: String,
        timestamp: i64,
    },
    /// IMU (Inertial Measurement Unit) acceleration data
    IMU {
        x: f32,
        y: f32,
        z: f32,
        timestamp: i64,
    },
    /// Room occupancy state
    Occupancy {
        occupied: bool,
        count: u32,
        last_motion: i64,
        timestamp: i64,
    },
}

/// Temperature sensor data point for plotting
#[derive(Debug, Clone)]
pub struct TemperatureData {
    pub value: f32,
    pub timestamp: f64,
}

/// IMU data point for plotting
#[derive(Debug, Clone)]
pub struct IMUData {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub timestamp: f64,
}

/// Occupancy state
#[derive(Debug, Clone)]
pub struct OccupancyData {
    pub occupied: bool,
    pub count: u32,
    pub last_motion: i64,
    pub timestamp: i64,
}

impl Default for OccupancyData {
    fn default() -> Self {
        Self {
            occupied: false,
            count: 0,
            last_motion: 0,
            timestamp: 0,
        }
    }
}
