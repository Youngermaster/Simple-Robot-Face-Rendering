//! WebSocket and HTTP server for sensor data
//!
//! Handles incoming sensor data via WebSocket (streaming) and HTTP (state updates).

use axum::{
    extract::{
        ws::{Message, WebSocket},
        State, WebSocketUpgrade,
    },
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use futures_util::StreamExt;
use serde_json::Value;
use std::sync::Arc;
use tokio::sync::broadcast;
use tower_http::cors::CorsLayer;
use tracing::{error, info, warn};

use crate::messages::SensorMessage;

/// Shared server state
#[derive(Clone)]
pub struct ServerState {
    pub sensor_tx: broadcast::Sender<SensorMessage>,
}

/// Starts the axum server on port 8080
pub async fn start_server(sensor_tx: broadcast::Sender<SensorMessage>) {
    let state = ServerState { sensor_tx };

    let app = Router::new()
        .route("/ws/temperature", get(temperature_websocket_handler))
        .route("/ws/imu", get(imu_websocket_handler))
        .route("/api/occupancy", post(occupancy_http_handler))
        .route("/health", get(health_check))
        .layer(CorsLayer::permissive())
        .with_state(Arc::new(state));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080")
        .await
        .unwrap();

    info!("🚀 Server listening on http://0.0.0.0:8080");
    info!("   WebSocket endpoints:");
    info!("     - ws://localhost:8080/ws/temperature");
    info!("     - ws://localhost:8080/ws/imu");
    info!("   HTTP endpoint:");
    info!("     - POST http://localhost:8080/api/occupancy");

    axum::serve(listener, app).await.unwrap();
}

/// Health check endpoint
async fn health_check() -> &'static str {
    "OK"
}

/// WebSocket handler for temperature sensor
async fn temperature_websocket_handler(
    ws: WebSocketUpgrade,
    State(state): State<Arc<ServerState>>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_temperature_socket(socket, state))
}

/// WebSocket handler for IMU sensor
async fn imu_websocket_handler(
    ws: WebSocketUpgrade,
    State(state): State<Arc<ServerState>>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_imu_socket(socket, state))
}

/// HTTP handler for occupancy updates
async fn occupancy_http_handler(
    State(state): State<Arc<ServerState>>,
    Json(payload): Json<Value>,
) -> Result<StatusCode, StatusCode> {
    // Parse occupancy data
    let occupied = payload["occupied"].as_bool().unwrap_or(false);
    let count = payload["count"].as_u64().unwrap_or(0) as u32;
    let timestamp = chrono::Utc::now().timestamp_millis();

    let message = SensorMessage::Occupancy {
        occupied,
        count,
        last_motion: timestamp,
        timestamp,
    };

    // Broadcast to GUI
    if let Err(e) = state.sensor_tx.send(message) {
        error!("Failed to broadcast occupancy: {}", e);
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }

    info!("📍 Occupancy updated: occupied={}, count={}", occupied, count);
    Ok(StatusCode::OK)
}

/// Handle temperature WebSocket connection
async fn handle_temperature_socket(socket: WebSocket, state: Arc<ServerState>) {
    let (_sender, mut receiver) = socket.split();

    info!("🌡️  Temperature sensor connected");

    while let Some(msg) = receiver.next().await {
        match msg {
            Ok(Message::Text(text)) => {
                match serde_json::from_str::<Value>(&text) {
                    Ok(data) => {
                        let value = data["value"].as_f64().unwrap_or(0.0) as f32;
                        let unit = data["unit"].as_str().unwrap_or("C").to_string();
                        let timestamp = chrono::Utc::now().timestamp_millis();

                        let message = SensorMessage::Temperature {
                            value,
                            unit,
                            timestamp,
                        };

                        // Broadcast to GUI
                        if let Err(e) = state.sensor_tx.send(message) {
                            error!("Failed to broadcast temperature: {}", e);
                        }
                    }
                    Err(e) => {
                        warn!("Failed to parse temperature data: {}", e);
                    }
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

/// Handle IMU WebSocket connection
async fn handle_imu_socket(socket: WebSocket, state: Arc<ServerState>) {
    let (_sender, mut receiver) = socket.split();

    info!("📊 IMU sensor connected");

    while let Some(msg) = receiver.next().await {
        match msg {
            Ok(Message::Text(text)) => {
                match serde_json::from_str::<Value>(&text) {
                    Ok(data) => {
                        let x = data["x"].as_f64().unwrap_or(0.0) as f32;
                        let y = data["y"].as_f64().unwrap_or(0.0) as f32;
                        let z = data["z"].as_f64().unwrap_or(0.0) as f32;
                        let timestamp = chrono::Utc::now().timestamp_millis();

                        let message = SensorMessage::IMU { x, y, z, timestamp };

                        // Broadcast to GUI
                        if let Err(e) = state.sensor_tx.send(message) {
                            error!("Failed to broadcast IMU: {}", e);
                        }
                    }
                    Err(e) => {
                        warn!("Failed to parse IMU data: {}", e);
                    }
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
