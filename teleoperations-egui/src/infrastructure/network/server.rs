//! Network server implementation
//!
//! Sets up and runs the axum server with WebSocket and HTTP endpoints.

use axum::{
    extract::WebSocketUpgrade,
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use std::sync::Arc;
use tower_http::cors::CorsLayer;
use tracing::info;

use crate::application::{AppConfig, TelemetryService};
use super::handlers::{
    handle_imu_websocket, handle_occupancy_http, handle_temperature_websocket, health_check,
    ServerState,
};

/// Network server
pub struct Server {
    config: AppConfig,
    telemetry_service: Arc<TelemetryService>,
}

impl Server {
    /// Create a new server
    pub fn new(config: AppConfig, telemetry_service: Arc<TelemetryService>) -> Self {
        Self {
            config,
            telemetry_service,
        }
    }

    /// Start the server
    pub async fn start(self) {
        let state = Arc::new(ServerState {
            telemetry_service: self.telemetry_service,
        });

        let app = Router::new()
            .route("/ws/temperature", get(temperature_websocket_handler))
            .route("/ws/imu", get(imu_websocket_handler))
            .route("/api/occupancy", post(handle_occupancy_http))
            .route("/health", get(health_check))
            .layer(CorsLayer::permissive())
            .with_state(state);

        let addr = format!("{}:{}", self.config.server.host, self.config.server.port);
        let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

        info!("🚀 Server listening on http://{}", addr);
        info!("   WebSocket endpoints:");
        info!("     - ws://{}:{}/ws/temperature", self.config.server.host, self.config.server.port);
        info!("     - ws://{}:{}/ws/imu", self.config.server.host, self.config.server.port);
        info!("   HTTP endpoint:");
        info!("     - POST http://{}:{}/api/occupancy", self.config.server.host, self.config.server.port);

        axum::serve(listener, app).await.unwrap();
    }
}

// ============================================================================
// WebSocket upgrade handlers
// ============================================================================

async fn temperature_websocket_handler(
    ws: WebSocketUpgrade,
    axum::extract::State(state): axum::extract::State<Arc<ServerState>>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_temperature_websocket(socket, state))
}

async fn imu_websocket_handler(
    ws: WebSocketUpgrade,
    axum::extract::State(state): axum::extract::State<Arc<ServerState>>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_imu_websocket(socket, state))
}
