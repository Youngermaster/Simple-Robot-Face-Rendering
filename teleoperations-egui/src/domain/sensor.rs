//! Sensor domain models
//!
//! Defines the core sensor abstraction and types.
//! This is framework-agnostic and represents pure business logic.

use serde::{Deserialize, Serialize};

/// Types of sensors supported by the system
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SensorType {
    Temperature,
    IMU,
    Occupancy,
}

/// Generic sensor reading trait
pub trait SensorReading: Clone + Send + Sync {
    /// Get the sensor type
    fn sensor_type(&self) -> SensorType;

    /// Get the timestamp in milliseconds
    fn timestamp(&self) -> i64;

    /// Validate the reading (e.g., check if values are within acceptable ranges)
    fn validate(&self) -> Result<(), String>;
}

/// Temperature sensor reading
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemperatureReading {
    pub value: f32,
    pub unit: TemperatureUnit,
    pub timestamp: i64,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum TemperatureUnit {
    Celsius,
    Fahrenheit,
}

impl SensorReading for TemperatureReading {
    fn sensor_type(&self) -> SensorType {
        SensorType::Temperature
    }

    fn timestamp(&self) -> i64 {
        self.timestamp
    }

    fn validate(&self) -> Result<(), String> {
        // Validate temperature is within reasonable bounds
        if self.value < -273.15 {
            return Err("Temperature below absolute zero".to_string());
        }
        if self.value > 1000.0 {
            return Err("Temperature unreasonably high".to_string());
        }
        Ok(())
    }
}

/// IMU (Inertial Measurement Unit) reading
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IMUReading {
    pub acceleration: Vector3,
    pub timestamp: i64,
}

/// 3D vector for acceleration data
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn magnitude(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }
}

impl SensorReading for IMUReading {
    fn sensor_type(&self) -> SensorType {
        SensorType::IMU
    }

    fn timestamp(&self) -> i64 {
        self.timestamp
    }

    fn validate(&self) -> Result<(), String> {
        // Validate acceleration is within reasonable bounds (e.g., < 100 m/s²)
        let magnitude = self.acceleration.magnitude();
        if magnitude > 100.0 {
            return Err(format!("Acceleration too high: {:.2} m/s²", magnitude));
        }
        Ok(())
    }
}

/// Occupancy sensor reading
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OccupancyReading {
    pub occupied: bool,
    pub count: u32,
    pub last_motion_time: i64,
    pub timestamp: i64,
}

impl SensorReading for OccupancyReading {
    fn sensor_type(&self) -> SensorType {
        SensorType::Occupancy
    }

    fn timestamp(&self) -> i64 {
        self.timestamp
    }

    fn validate(&self) -> Result<(), String> {
        // Validate count is reasonable
        if self.count > 1000 {
            return Err(format!("Occupancy count too high: {}", self.count));
        }
        if self.count > 0 && !self.occupied {
            return Err("Count > 0 but occupied is false".to_string());
        }
        Ok(())
    }
}

/// Generic sensor trait for extensibility
pub trait Sensor: Send + Sync {
    /// Get the sensor type
    fn sensor_type(&self) -> SensorType;

    /// Get the sensor's unique identifier
    fn id(&self) -> &str;

    /// Check if the sensor is active/connected
    fn is_active(&self) -> bool;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_temperature_validation() {
        let valid = TemperatureReading {
            value: 25.0,
            unit: TemperatureUnit::Celsius,
            timestamp: 1000,
        };
        assert!(valid.validate().is_ok());

        let invalid = TemperatureReading {
            value: -300.0,
            unit: TemperatureUnit::Celsius,
            timestamp: 1000,
        };
        assert!(invalid.validate().is_err());
    }

    #[test]
    fn test_imu_validation() {
        let valid = IMUReading {
            acceleration: Vector3::new(0.5, 9.81, 0.2),
            timestamp: 1000,
        };
        assert!(valid.validate().is_ok());

        let invalid = IMUReading {
            acceleration: Vector3::new(200.0, 0.0, 0.0),
            timestamp: 1000,
        };
        assert!(invalid.validate().is_err());
    }

    #[test]
    fn test_vector3_magnitude() {
        let v = Vector3::new(3.0, 4.0, 0.0);
        assert_eq!(v.magnitude(), 5.0);
    }
}
