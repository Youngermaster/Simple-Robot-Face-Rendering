//! Telemetry domain models
//!
//! Represents telemetry data and events in the system.
//! This is the domain language for sensor data communication.

use super::sensor::{IMUReading, OccupancyReading, SensorReading, SensorType, TemperatureReading};

/// Unified telemetry data from all sensor types
#[derive(Debug, Clone)]
pub enum TelemetryData {
    Temperature(TemperatureReading),
    IMU(IMUReading),
    Occupancy(OccupancyReading),
}

impl TelemetryData {
    /// Get the sensor type of this telemetry data
    pub fn sensor_type(&self) -> SensorType {
        match self {
            TelemetryData::Temperature(_) => SensorType::Temperature,
            TelemetryData::IMU(_) => SensorType::IMU,
            TelemetryData::Occupancy(_) => SensorType::Occupancy,
        }
    }

    /// Get the timestamp of this telemetry data
    pub fn timestamp(&self) -> i64 {
        match self {
            TelemetryData::Temperature(r) => r.timestamp,
            TelemetryData::IMU(r) => r.timestamp,
            TelemetryData::Occupancy(r) => r.timestamp,
        }
    }

    /// Validate the telemetry data
    pub fn validate(&self) -> Result<(), String> {
        match self {
            TelemetryData::Temperature(r) => r.validate(),
            TelemetryData::IMU(r) => r.validate(),
            TelemetryData::Occupancy(r) => r.validate(),
        }
    }
}

/// Events that occur in the telemetry system
#[derive(Debug, Clone)]
pub enum TelemetryEvent {
    /// New sensor data received
    DataReceived {
        sensor_type: SensorType,
        data: TelemetryData,
    },

    /// Sensor connected
    SensorConnected {
        sensor_type: SensorType,
        sensor_id: String,
    },

    /// Sensor disconnected
    SensorDisconnected {
        sensor_type: SensorType,
        sensor_id: String,
    },

    /// Data validation failed
    ValidationFailed {
        sensor_type: SensorType,
        error: String,
    },
}

/// Telemetry statistics for monitoring
#[derive(Debug, Clone, Default)]
pub struct TelemetryStats {
    pub total_messages: u64,
    pub messages_per_sensor: [u64; 3], // Temperature, IMU, Occupancy
    pub validation_errors: u64,
}

impl TelemetryStats {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn record_message(&mut self, sensor_type: SensorType) {
        self.total_messages += 1;
        match sensor_type {
            SensorType::Temperature => self.messages_per_sensor[0] += 1,
            SensorType::IMU => self.messages_per_sensor[1] += 1,
            SensorType::Occupancy => self.messages_per_sensor[2] += 1,
        }
    }

    pub fn record_validation_error(&mut self) {
        self.validation_errors += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::sensor::{TemperatureUnit, Vector3};

    #[test]
    fn test_telemetry_data_sensor_type() {
        let temp_data = TelemetryData::Temperature(TemperatureReading {
            value: 25.0,
            unit: TemperatureUnit::Celsius,
            timestamp: 1000,
        });
        assert_eq!(temp_data.sensor_type(), SensorType::Temperature);
    }

    #[test]
    fn test_telemetry_stats() {
        let mut stats = TelemetryStats::new();
        stats.record_message(SensorType::Temperature);
        stats.record_message(SensorType::IMU);

        assert_eq!(stats.total_messages, 2);
        assert_eq!(stats.messages_per_sensor[0], 1);
        assert_eq!(stats.messages_per_sensor[1], 1);
    }
}
