//! Network request handlers
//!
//! Handles WebSocket and HTTP requests, converting them to domain types.

use axum::{
    extract::{
        ws::{Message, WebSocket},
        State,
    },
    http::StatusCode,
    Json,
};
use futures_util::StreamExt;
use serde_json::Value;
use std::sync::Arc;
use tracing::{error, info, warn};

use crate::application::TelemetryService;
use crate::domain::sensor::{
    IMUReading, OccupancyReading, TemperatureReading, TemperatureUnit, Vector3,
};
use crate::domain::telemetry::TelemetryData;

/// Shared state for all handlers
#[derive(Clone)]
pub struct ServerState {
    pub telemetry_service: Arc<TelemetryService>,
}

/// Handle temperature WebSocket connections
pub async fn handle_temperature_websocket(socket: WebSocket, state: Arc<ServerState>) {
    let (_sender, mut receiver) = socket.split();

    info!("🌡️  Temperature sensor connected");

    while let Some(msg) = receiver.next().await {
        match msg {
            Ok(Message::Text(text)) => {
                if let Err(e) = process_temperature_message(&text, &state) {
                    warn!("Failed to process temperature message: {}", e);
                }
            }
            Ok(Message::Close(_)) => {
                info!("🌡️  Temperature sensor disconnected");
                break;
            }
            Err(e) => {
                error!("WebSocket error: {}", e);
                break;
            }
            _ => {}
        }
    }
}

/// Handle IMU WebSocket connections
pub async fn handle_imu_websocket(socket: WebSocket, state: Arc<ServerState>) {
    let (_sender, mut receiver) = socket.split();

    info!("📊 IMU sensor connected");

    while let Some(msg) = receiver.next().await {
        match msg {
            Ok(Message::Text(text)) => {
                if let Err(e) = process_imu_message(&text, &state) {
                    warn!("Failed to process IMU message: {}", e);
                }
            }
            Ok(Message::Close(_)) => {
                info!("📊 IMU sensor disconnected");
                break;
            }
            Err(e) => {
                error!("WebSocket error: {}", e);
                break;
            }
            _ => {}
        }
    }
}

/// Handle occupancy HTTP POST requests
pub async fn handle_occupancy_http(
    State(state): State<Arc<ServerState>>,
    Json(payload): Json<Value>,
) -> Result<StatusCode, StatusCode> {
    match process_occupancy_message(&payload, &state) {
        Ok(_) => {
            info!("📍 Occupancy updated successfully");
            Ok(StatusCode::OK)
        }
        Err(e) => {
            error!("Failed to process occupancy: {}", e);
            Err(StatusCode::BAD_REQUEST)
        }
    }
}

/// Health check endpoint
pub async fn health_check() -> &'static str {
    "OK"
}

// ============================================================================
// Private message processing functions
// ============================================================================

fn process_temperature_message(text: &str, state: &ServerState) -> Result<(), String> {
    let data: Value =
        serde_json::from_str(text).map_err(|e| format!("JSON parse error: {}", e))?;

    let value = data["value"]
        .as_f64()
        .ok_or("Missing 'value' field")?
        as f32;

    let unit_str = data["unit"].as_str().unwrap_or("C");
    let unit = match unit_str {
        "F" | "Fahrenheit" => TemperatureUnit::Fahrenheit,
        _ => TemperatureUnit::Celsius,
    };

    let reading = TemperatureReading {
        value,
        unit,
        timestamp: chrono::Utc::now().timestamp_millis(),
    };

    state
        .telemetry_service
        .publish_data(TelemetryData::Temperature(reading))
}

fn process_imu_message(text: &str, state: &ServerState) -> Result<(), String> {
    let data: Value =
        serde_json::from_str(text).map_err(|e| format!("JSON parse error: {}", e))?;

    let x = data["x"].as_f64().ok_or("Missing 'x' field")? as f32;
    let y = data["y"].as_f64().ok_or("Missing 'y' field")? as f32;
    let z = data["z"].as_f64().ok_or("Missing 'z' field")? as f32;

    let reading = IMUReading {
        acceleration: Vector3::new(x, y, z),
        timestamp: chrono::Utc::now().timestamp_millis(),
    };

    state
        .telemetry_service
        .publish_data(TelemetryData::IMU(reading))
}

fn process_occupancy_message(payload: &Value, state: &ServerState) -> Result<(), String> {
    let occupied = payload["occupied"]
        .as_bool()
        .ok_or("Missing 'occupied' field")?;

    let count = payload["count"].as_u64().unwrap_or(0) as u32;

    let timestamp = chrono::Utc::now().timestamp_millis();

    let reading = OccupancyReading {
        occupied,
        count,
        last_motion_time: timestamp,
        timestamp,
    };

    state
        .telemetry_service
        .publish_data(TelemetryData::Occupancy(reading))
}
