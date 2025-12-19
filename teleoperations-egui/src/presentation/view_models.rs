//! View Models
//!
//! Transforms domain models into UI-friendly formats.
//! This decouples the presentation layer from domain changes.

use std::collections::VecDeque;

/// Temperature data optimized for plotting
#[derive(Debug, Clone)]
pub struct TemperatureViewModel {
    pub value: f32,
    pub timestamp: f64,
}

/// IMU data optimized for plotting
#[derive(Debug, Clone)]
pub struct IMUViewModel {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub timestamp: f64,
}

/// Occupancy data for display
#[derive(Debug, Clone, Default)]
pub struct OccupancyViewModel {
    pub occupied: bool,
    pub count: u32,
    pub last_motion_time: i64,
    pub timestamp: i64,
}

impl OccupancyViewModel {
    pub fn is_active(&self) -> bool {
        self.timestamp > 0
    }

    pub fn get_status_text(&self) -> &'static str {
        if self.occupied {
            "🟢 OCCUPIED"
        } else {
            "🔴 VACANT"
        }
    }

    pub fn get_status_color(&self) -> egui::Color32 {
        if self.occupied {
            egui::Color32::from_rgb(100, 255, 100)
        } else {
            egui::Color32::from_rgb(255, 100, 100)
        }
    }
}

/// Container for all panel data
#[derive(Debug)]
pub struct PanelData {
    pub temperature: VecDeque<TemperatureViewModel>,
    pub imu_x: VecDeque<IMUViewModel>,
    pub imu_y: VecDeque<IMUViewModel>,
    pub imu_z: VecDeque<IMUViewModel>,
    pub occupancy: OccupancyViewModel,
}

impl PanelData {
    pub fn new(capacity: usize) -> Self {
        Self {
            temperature: VecDeque::with_capacity(capacity),
            imu_x: VecDeque::with_capacity(capacity),
            imu_y: VecDeque::with_capacity(capacity),
            imu_z: VecDeque::with_capacity(capacity),
            occupancy: OccupancyViewModel::default(),
        }
    }
}
