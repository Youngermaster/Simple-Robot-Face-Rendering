//! # Teleoperation Monitoring System
//!
//! A professional robotics monitoring dashboard built with egui.
//! Features real-time sensor visualization similar to Foxglove/Rerun.
//!
//! ## Architecture
//! - **Backend**: Axum server with WebSocket + HTTP endpoints
//! - **Frontend**: eframe/egui with real-time plotting
//! - **Communication**: Tokio broadcast channels
//!
//! ## Sensors
//! 1. Temperature (WebSocket) - Real-time temperature graph
//! 2. IMU (WebSocket) - 3-axis acceleration plot
//! 3. Occupancy (HTTP) - Room state display

#![warn(clippy::all, rust_2018_idioms)]

mod messages;
mod server;

use eframe::egui;
use egui_plot::{Line, Plot, PlotPoints};
use messages::*;
use std::collections::VecDeque;
use tokio::sync::broadcast;
use tracing::{info, Level};
use tracing_subscriber;

const MAX_DATA_POINTS: usize = 200; // Keep last 200 points for plotting

fn main() -> eframe::Result {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .init();

    info!("🤖 Starting Teleoperation Monitoring System");

    // Create broadcast channel for sensor data
    let (sensor_tx, _sensor_rx) = broadcast::channel::<SensorMessage>(100);

    // Start server in background thread
    let server_tx = sensor_tx.clone();
    std::thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            server::start_server(server_tx).await;
        });
    });

    // Give server time to start
    std::thread::sleep(std::time::Duration::from_millis(500));

    // Create and run egui app
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1400.0, 900.0])
            .with_title("🤖 Teleoperation Monitoring"),
        ..Default::default()
    };

    eframe::run_native(
        "Teleoperation Monitoring",
        options,
        Box::new(|cc| Ok(Box::new(TeleoperationApp::new(cc, sensor_tx)))),
    )
}

/// Main application state
struct TeleoperationApp {
    // Data buffers
    temperature_data: VecDeque<TemperatureData>,
    imu_data_x: VecDeque<IMUData>,
    imu_data_y: VecDeque<IMUData>,
    imu_data_z: VecDeque<IMUData>,
    occupancy_data: OccupancyData,

    // Channel receiver
    sensor_rx: broadcast::Receiver<SensorMessage>,

    // UI state
    start_time: std::time::Instant,
    temperature_min: f32,
    temperature_max: f32,
    imu_range: f32,
}

impl TeleoperationApp {
    fn new(_cc: &eframe::CreationContext<'_>, sensor_tx: broadcast::Sender<SensorMessage>) -> Self {
        let sensor_rx = sensor_tx.subscribe();

        Self {
            temperature_data: VecDeque::with_capacity(MAX_DATA_POINTS),
            imu_data_x: VecDeque::with_capacity(MAX_DATA_POINTS),
            imu_data_y: VecDeque::with_capacity(MAX_DATA_POINTS),
            imu_data_z: VecDeque::with_capacity(MAX_DATA_POINTS),
            occupancy_data: OccupancyData::default(),
            sensor_rx,
            start_time: std::time::Instant::now(),
            temperature_min: 15.0,
            temperature_max: 35.0,
            imu_range: 2.0,
        }
    }

    /// Process incoming sensor messages
    fn process_messages(&mut self) {
        while let Ok(msg) = self.sensor_rx.try_recv() {
            match msg {
                SensorMessage::Temperature {
                    value,
                    unit: _,
                    timestamp,
                } => {
                    let time = self.get_relative_time(timestamp);
                    self.temperature_data.push_back(TemperatureData {
                        value,
                        timestamp: time,
                    });

                    // Keep only last MAX_DATA_POINTS
                    if self.temperature_data.len() > MAX_DATA_POINTS {
                        self.temperature_data.pop_front();
                    }

                    // Auto-adjust range
                    self.temperature_min = self.temperature_min.min(value - 2.0);
                    self.temperature_max = self.temperature_max.max(value + 2.0);
                }
                SensorMessage::IMU { x, y, z, timestamp } => {
                    let time = self.get_relative_time(timestamp);

                    self.imu_data_x.push_back(IMUData {
                        x,
                        y,
                        z,
                        timestamp: time,
                    });
                    self.imu_data_y.push_back(IMUData {
                        x,
                        y,
                        z,
                        timestamp: time,
                    });
                    self.imu_data_z.push_back(IMUData {
                        x,
                        y,
                        z,
                        timestamp: time,
                    });

                    // Keep only last MAX_DATA_POINTS
                    if self.imu_data_x.len() > MAX_DATA_POINTS {
                        self.imu_data_x.pop_front();
                        self.imu_data_y.pop_front();
                        self.imu_data_z.pop_front();
                    }

                    // Auto-adjust range
                    let max_val = x.abs().max(y.abs()).max(z.abs());
                    self.imu_range = self.imu_range.max(max_val + 0.5);
                }
                SensorMessage::Occupancy {
                    occupied,
                    count,
                    last_motion,
                    timestamp,
                } => {
                    self.occupancy_data = OccupancyData {
                        occupied,
                        count,
                        last_motion,
                        timestamp,
                    };
                }
            }
        }
    }

    /// Convert absolute timestamp to relative time in seconds
    fn get_relative_time(&self, _timestamp: i64) -> f64 {
        self.start_time.elapsed().as_secs_f64()
    }

    /// Render temperature panel
    fn render_temperature_panel(&mut self, ui: &mut egui::Ui) {
        ui.heading("🌡️ Temperature Sensor");

        // Stats
        if let Some(latest) = self.temperature_data.back() {
            ui.horizontal(|ui| {
                ui.label("Current:");
                ui.colored_label(egui::Color32::from_rgb(255, 150, 50), format!("{:.1}°C", latest.value));
            });
        } else {
            ui.colored_label(egui::Color32::GRAY, "No data");
        }

        ui.add_space(5.0);

        // Plot
        Plot::new("temperature_plot")
            .height(200.0)
            .show_axes([true, true])
            .allow_zoom(true)
            .allow_drag(true)
            .show(ui, |plot_ui| {
                let points: PlotPoints = self
                    .temperature_data
                    .iter()
                    .map(|d| [d.timestamp, d.value as f64])
                    .collect();

                plot_ui.line(
                    Line::new(points)
                        .color(egui::Color32::from_rgb(255, 100, 100))
                        .width(2.0),
                );
            });

        ui.add_space(5.0);
        ui.label(format!("Data points: {}", self.temperature_data.len()));
    }

    /// Render IMU panel
    fn render_imu_panel(&mut self, ui: &mut egui::Ui) {
        ui.heading("📊 IMU Accelerometer");

        // Stats
        if let Some(latest) = self.imu_data_x.back() {
            ui.horizontal(|ui| {
                ui.label("X:");
                ui.colored_label(egui::Color32::from_rgb(255, 100, 100), format!("{:.2}", latest.x));
                ui.label("Y:");
                ui.colored_label(egui::Color32::from_rgb(100, 255, 100), format!("{:.2}", latest.y));
                ui.label("Z:");
                ui.colored_label(egui::Color32::from_rgb(100, 100, 255), format!("{:.2}", latest.z));
            });
        } else {
            ui.colored_label(egui::Color32::GRAY, "No data");
        }

        ui.add_space(5.0);

        // Plot
        Plot::new("imu_plot")
            .height(200.0)
            .show_axes([true, true])
            .allow_zoom(true)
            .allow_drag(true)
            .show(ui, |plot_ui| {
                // X axis
                let points_x: PlotPoints = self
                    .imu_data_x
                    .iter()
                    .map(|d| [d.timestamp, d.x as f64])
                    .collect();
                plot_ui.line(
                    Line::new(points_x)
                        .color(egui::Color32::from_rgb(255, 100, 100))
                        .width(2.0)
                        .name("X"),
                );

                // Y axis
                let points_y: PlotPoints = self
                    .imu_data_y
                    .iter()
                    .map(|d| [d.timestamp, d.y as f64])
                    .collect();
                plot_ui.line(
                    Line::new(points_y)
                        .color(egui::Color32::from_rgb(100, 255, 100))
                        .width(2.0)
                        .name("Y"),
                );

                // Z axis
                let points_z: PlotPoints = self
                    .imu_data_z
                    .iter()
                    .map(|d| [d.timestamp, d.z as f64])
                    .collect();
                plot_ui.line(
                    Line::new(points_z)
                        .color(egui::Color32::from_rgb(100, 100, 255))
                        .width(2.0)
                        .name("Z"),
                );
            });

        ui.add_space(5.0);
        ui.label(format!("Data points: {}", self.imu_data_x.len()));
    }

    /// Render occupancy panel
    fn render_occupancy_panel(&mut self, ui: &mut egui::Ui) {
        ui.heading("🚪 Room Occupancy");

        ui.add_space(10.0);

        // Occupancy status
        let status_color = if self.occupancy_data.occupied {
            egui::Color32::from_rgb(100, 255, 100)
        } else {
            egui::Color32::from_rgb(255, 100, 100)
        };

        let status_text = if self.occupancy_data.occupied {
            "🟢 OCCUPIED"
        } else {
            "🔴 VACANT"
        };

        ui.horizontal(|ui| {
            ui.label("Status:");
            ui.colored_label(status_color, status_text);
        });

        ui.add_space(10.0);

        // People count
        ui.horizontal(|ui| {
            ui.label("People Count:");
            ui.colored_label(
                egui::Color32::WHITE,
                format!("{}", self.occupancy_data.count),
            );
        });

        ui.add_space(10.0);

        // Last motion
        if self.occupancy_data.last_motion > 0 {
            let elapsed = (chrono::Utc::now().timestamp_millis() - self.occupancy_data.last_motion)
                / 1000;
            ui.horizontal(|ui| {
                ui.label("Last Motion:");
                ui.colored_label(egui::Color32::LIGHT_GRAY, format!("{}s ago", elapsed));
            });
        }

        ui.add_space(20.0);

        // Visual indicator
        let rect_size = egui::vec2(200.0, 100.0);
        let (rect, _response) = ui.allocate_exact_size(rect_size, egui::Sense::hover());

        if ui.is_rect_visible(rect) {
            let painter = ui.painter();
            painter.rect_filled(rect, 5.0, status_color.linear_multiply(0.3));
            painter.rect_stroke(
                rect,
                5.0,
                egui::Stroke::new(2.0, status_color),
            );

            // Draw icon
            let center = rect.center();
            let icon = if self.occupancy_data.occupied {
                "👥"
            } else {
                "🚫"
            };

            painter.text(
                center,
                egui::Align2::CENTER_CENTER,
                icon,
                egui::FontId::proportional(48.0),
                egui::Color32::WHITE,
            );
        }
    }
}

impl eframe::App for TeleoperationApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Process incoming sensor messages
        self.process_messages();

        // Request continuous repaint for real-time updates
        ctx.request_repaint();

        // Top panel - Header
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.add_space(5.0);
            ui.horizontal(|ui| {
                ui.heading("🤖 Teleoperation Monitoring System");
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(format!("Runtime: {:.1}s", self.start_time.elapsed().as_secs_f32()));
                });
            });
            ui.add_space(5.0);
        });

        // Main content - 3 panels
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add_space(10.0);

            egui::ScrollArea::vertical().show(ui, |ui| {
                // Temperature Panel
                egui::Frame::group(ui.style())
                    .fill(egui::Color32::from_rgb(30, 30, 35))
                    .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(60, 60, 70)))
                    .inner_margin(15.0)
                    .show(ui, |ui| {
                        self.render_temperature_panel(ui);
                    });

                ui.add_space(15.0);

                // IMU Panel
                egui::Frame::group(ui.style())
                    .fill(egui::Color32::from_rgb(30, 30, 35))
                    .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(60, 60, 70)))
                    .inner_margin(15.0)
                    .show(ui, |ui| {
                        self.render_imu_panel(ui);
                    });

                ui.add_space(15.0);

                // Occupancy Panel
                egui::Frame::group(ui.style())
                    .fill(egui::Color32::from_rgb(30, 30, 35))
                    .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(60, 60, 70)))
                    .inner_margin(15.0)
                    .show(ui, |ui| {
                        self.render_occupancy_panel(ui);
                    });

                ui.add_space(20.0);
            });
        });

        // Bottom panel - Connection info
        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            ui.add_space(5.0);
            ui.horizontal(|ui| {
                ui.label("Server:");
                ui.colored_label(egui::Color32::from_rgb(100, 255, 100), "http://localhost:8080");
                ui.separator();
                ui.label(format!(
                    "Temp: {} | IMU: {} | Occupancy: {}",
                    if self.temperature_data.is_empty() { "❌" } else { "✓" },
                    if self.imu_data_x.is_empty() { "❌" } else { "✓" },
                    if self.occupancy_data.timestamp > 0 { "✓" } else { "❌" }
                ));
            });
            ui.add_space(5.0);
        });
    }
}
