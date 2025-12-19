//! Telemetry Service
//!
//! Core service for managing telemetry data collection and distribution.
//! This is the main use case coordinator.

use crate::domain::telemetry::{TelemetryData, TelemetryEvent, TelemetryStats};
use std::sync::{Arc, Mutex};
use tokio::sync::broadcast;
use tracing::warn;

/// Telemetry service handles data collection and distribution
pub struct TelemetryService {
    /// Broadcast channel for telemetry data
    data_tx: broadcast::Sender<TelemetryData>,

    /// Broadcast channel for telemetry events
    event_tx: broadcast::Sender<TelemetryEvent>,

    /// Statistics tracker
    stats: Arc<Mutex<TelemetryStats>>,

    /// Configuration
    config: TelemetryServiceConfig,
}

#[derive(Debug, Clone)]
pub struct TelemetryServiceConfig {
    pub channel_capacity: usize,
    pub enable_validation: bool,
}

impl Default for TelemetryServiceConfig {
    fn default() -> Self {
        Self {
            channel_capacity: 100,
            enable_validation: true,
        }
    }
}

impl TelemetryService {
    /// Create a new telemetry service
    pub fn new(config: TelemetryServiceConfig) -> Self {
        let (data_tx, _) = broadcast::channel(config.channel_capacity);
        let (event_tx, _) = broadcast::channel(config.channel_capacity);

        Self {
            data_tx,
            event_tx,
            stats: Arc::new(Mutex::new(TelemetryStats::new())),
            config,
        }
    }

    /// Subscribe to telemetry data
    pub fn subscribe_data(&self) -> broadcast::Receiver<TelemetryData> {
        self.data_tx.subscribe()
    }

    /// Subscribe to telemetry events
    pub fn subscribe_events(&self) -> broadcast::Receiver<TelemetryEvent> {
        self.event_tx.subscribe()
    }

    /// Get a clone of the data sender (for infrastructure layer)
    pub fn get_data_sender(&self) -> broadcast::Sender<TelemetryData> {
        self.data_tx.clone()
    }

    /// Publish telemetry data
    pub fn publish_data(&self, data: TelemetryData) -> Result<(), String> {
        // Validate if enabled
        if self.config.enable_validation {
            if let Err(e) = data.validate() {
                warn!("Validation failed: {}", e);

                // Update stats
                if let Ok(mut stats) = self.stats.lock() {
                    stats.record_validation_error();
                }

                // Publish validation error event
                let _ = self.event_tx.send(TelemetryEvent::ValidationFailed {
                    sensor_type: data.sensor_type(),
                    error: e.clone(),
                });

                return Err(e);
            }
        }

        // Update stats
        if let Ok(mut stats) = self.stats.lock() {
            stats.record_message(data.sensor_type());
        }

        // Publish data
        self.data_tx
            .send(data.clone())
            .map_err(|e| format!("Failed to publish data: {}", e))?;

        // Publish data received event
        let _ = self.event_tx.send(TelemetryEvent::DataReceived {
            sensor_type: data.sensor_type(),
            data,
        });

        Ok(())
    }

    /// Get current statistics
    pub fn get_stats(&self) -> TelemetryStats {
        self.stats.lock().unwrap().clone()
    }

    /// Reset statistics
    pub fn reset_stats(&self) {
        if let Ok(mut stats) = self.stats.lock() {
            *stats = TelemetryStats::new();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::sensor::{TemperatureReading, TemperatureUnit};

    #[test]
    fn test_telemetry_service_publish() {
        let service = TelemetryService::new(TelemetryServiceConfig::default());
        let mut rx = service.subscribe_data();

        let reading = TemperatureReading {
            value: 25.0,
            unit: TemperatureUnit::Celsius,
            timestamp: 1000,
        };

        let result = service.publish_data(TelemetryData::Temperature(reading.clone()));
        assert!(result.is_ok());

        // Check we received the data
        let received = rx.try_recv();
        assert!(received.is_ok());
    }

    #[test]
    fn test_telemetry_service_validation() {
        let service = TelemetryService::new(TelemetryServiceConfig {
            channel_capacity: 10,
            enable_validation: true,
        });

        // Invalid reading (below absolute zero)
        let invalid = TemperatureReading {
            value: -300.0,
            unit: TemperatureUnit::Celsius,
            timestamp: 1000,
        };

        let result = service.publish_data(TelemetryData::Temperature(invalid));
        assert!(result.is_err());

        // Check stats recorded the error
        let stats = service.get_stats();
        assert_eq!(stats.validation_errors, 1);
    }
}
