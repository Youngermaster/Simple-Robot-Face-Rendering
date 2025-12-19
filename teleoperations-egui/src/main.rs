//! Main Application Entry Point
//!
//! This is the orchestration layer that wires everything together.
//! It's intentionally minimal - all logic lives in the library modules.

use std::sync::Arc;
use tracing::Level;

use teleoperations_egui::{AppConfig, Server, TeleoperationApp, TelemetryService};

fn main() -> eframe::Result {
    // Initialize logging
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    tracing::info!("🤖 Starting Teleoperation Monitoring System");

    // Load configuration
    let config = AppConfig::default();

    // Create telemetry service (application layer)
    let telemetry_service = Arc::new(TelemetryService::new(
        teleoperations_egui::application::telemetry_service::TelemetryServiceConfig {
            channel_capacity: config.telemetry.channel_capacity,
            enable_validation: config.telemetry.enable_validation,
        },
    ));

    // Start server in background thread (infrastructure layer)
    let server_config = config.clone();
    let server_telemetry = telemetry_service.clone();
    std::thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let server = Server::new(server_config, server_telemetry);
            server.start().await;
        });
    });

    // Give server time to start
    std::thread::sleep(std::time::Duration::from_millis(500));

    // Create and run egui app (presentation layer)
    let app_config = config.clone();
    let app_telemetry = telemetry_service.clone();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([app_config.ui.window_width, app_config.ui.window_height])
            .with_title("🤖 Teleoperation Monitoring"),
        ..Default::default()
    };

    eframe::run_native(
        "Teleoperation Monitoring",
        options,
        Box::new(move |cc| {
            Ok(Box::new(TeleoperationApp::new(
                cc,
                app_config,
                &app_telemetry,
            )))
        }),
    )
}
