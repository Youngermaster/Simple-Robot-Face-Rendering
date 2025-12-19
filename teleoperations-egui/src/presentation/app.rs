//! Main Application
//!
//! Orchestrates the UI and coordinates data flow between services and panels.

use eframe::egui;
use std::time::Instant;
use tokio::sync::broadcast;

use crate::application::{AppConfig, TelemetryService};
use crate::domain::telemetry::TelemetryData;
use crate::presentation::panels::{IMUPanel, OccupancyPanel, TemperaturePanel};
use crate::presentation::view_models::{
    IMUViewModel, OccupancyViewModel, PanelData, TemperatureViewModel,
};

/// Main teleoperation application
pub struct TeleoperationApp {
    /// Panel data (view models)
    panel_data: PanelData,

    /// Telemetry data receiver
    telemetry_rx: broadcast::Receiver<TelemetryData>,

    /// Application start time for relative timestamps
    start_time: Instant,

    /// Configuration
    config: AppConfig,
}

impl TeleoperationApp {
    /// Create a new application
    pub fn new(
        _cc: &eframe::CreationContext<'_>,
        config: AppConfig,
        telemetry_service: &TelemetryService,
    ) -> Self {
        let telemetry_rx = telemetry_service.subscribe_data();

        Self {
            panel_data: PanelData::new(config.ui.max_data_points),
            telemetry_rx,
            start_time: Instant::now(),
            config,
        }
    }

    /// Process incoming telemetry messages
    fn process_telemetry(&mut self) {
        while let Ok(data) = self.telemetry_rx.try_recv() {
            match data {
                TelemetryData::Temperature(reading) => {
                    let view_model = TemperatureViewModel {
                        value: reading.value,
                        timestamp: self.start_time.elapsed().as_secs_f64(),
                    };

                    self.panel_data.temperature.push_back(view_model);

                    // Maintain max size
                    if self.panel_data.temperature.len() > self.config.ui.max_data_points {
                        self.panel_data.temperature.pop_front();
                    }
                }
                TelemetryData::IMU(reading) => {
                    let timestamp = self.start_time.elapsed().as_secs_f64();

                    let view_model = IMUViewModel {
                        x: reading.acceleration.x,
                        y: reading.acceleration.y,
                        z: reading.acceleration.z,
                        timestamp,
                    };

                    self.panel_data.imu_x.push_back(view_model.clone());
                    self.panel_data.imu_y.push_back(view_model.clone());
                    self.panel_data.imu_z.push_back(view_model);

                    // Maintain max size
                    if self.panel_data.imu_x.len() > self.config.ui.max_data_points {
                        self.panel_data.imu_x.pop_front();
                        self.panel_data.imu_y.pop_front();
                        self.panel_data.imu_z.pop_front();
                    }
                }
                TelemetryData::Occupancy(reading) => {
                    self.panel_data.occupancy = OccupancyViewModel {
                        occupied: reading.occupied,
                        count: reading.count,
                        last_motion_time: reading.last_motion_time,
                        timestamp: reading.timestamp,
                    };
                }
            }
        }
    }

    /// Render the top panel with header
    fn render_top_panel(&self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.add_space(5.0);
            ui.horizontal(|ui| {
                ui.heading("🤖 Teleoperation Monitoring System");
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(format!(
                        "Runtime: {:.1}s",
                        self.start_time.elapsed().as_secs_f32()
                    ));
                });
            });
            ui.add_space(5.0);
        });
    }

    /// Render the bottom panel with connection info
    fn render_bottom_panel(&self, ctx: &egui::Context) {
        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            ui.add_space(5.0);
            ui.horizontal(|ui| {
                ui.label("Server:");
                ui.colored_label(
                    egui::Color32::from_rgb(100, 255, 100),
                    format!("http://{}:{}", self.config.server.host, self.config.server.port),
                );
                ui.separator();
                ui.label(format!(
                    "Temp: {} | IMU: {} | Occupancy: {}",
                    if self.panel_data.temperature.is_empty() {
                        "❌"
                    } else {
                        "✓"
                    },
                    if self.panel_data.imu_x.is_empty() {
                        "❌"
                    } else {
                        "✓"
                    },
                    if self.panel_data.occupancy.is_active() {
                        "✓"
                    } else {
                        "❌"
                    }
                ));
            });
            ui.add_space(5.0);
        });
    }

    /// Render the main content with all panels
    fn render_central_panel(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add_space(10.0);

            egui::ScrollArea::vertical().show(ui, |ui| {
                // Temperature Panel
                Self::render_panel_frame(ui, |ui| {
                    TemperaturePanel::render(ui, &self.panel_data.temperature);
                });

                ui.add_space(15.0);

                // IMU Panel
                Self::render_panel_frame(ui, |ui| {
                    IMUPanel::render(
                        ui,
                        &self.panel_data.imu_x,
                        &self.panel_data.imu_y,
                        &self.panel_data.imu_z,
                    );
                });

                ui.add_space(15.0);

                // Occupancy Panel
                Self::render_panel_frame(ui, |ui| {
                    OccupancyPanel::render(ui, &self.panel_data.occupancy);
                });

                ui.add_space(20.0);
            });
        });
    }

    /// Helper to render a panel with consistent styling
    fn render_panel_frame(ui: &mut egui::Ui, content: impl FnOnce(&mut egui::Ui)) {
        egui::Frame::group(ui.style())
            .fill(egui::Color32::from_rgb(30, 30, 35))
            .stroke(egui::Stroke::new(
                1.0,
                egui::Color32::from_rgb(60, 60, 70),
            ))
            .inner_margin(15.0)
            .show(ui, content);
    }
}

impl eframe::App for TeleoperationApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Process incoming telemetry
        self.process_telemetry();

        // Request continuous repaint for real-time updates
        ctx.request_repaint();

        // Render UI
        self.render_top_panel(ctx);
        self.render_bottom_panel(ctx);
        self.render_central_panel(ctx);
    }
}
