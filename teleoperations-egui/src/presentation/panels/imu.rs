//! IMU Panel
//!
//! Displays 3-axis IMU accelerometer data with multi-line plots.

use egui::Ui;
use egui_plot::{Line, Plot, PlotPoints};
use std::collections::VecDeque;

use crate::presentation::view_models::IMUViewModel;

pub struct IMUPanel;

impl IMUPanel {
    pub fn render(
        ui: &mut Ui,
        data_x: &VecDeque<IMUViewModel>,
        data_y: &VecDeque<IMUViewModel>,
        data_z: &VecDeque<IMUViewModel>,
    ) {
        ui.heading("📊 IMU Accelerometer");

        // Current values display
        if let Some(latest) = data_x.back() {
            ui.horizontal(|ui| {
                ui.label("X:");
                ui.colored_label(
                    egui::Color32::from_rgb(255, 100, 100),
                    format!("{:.2}", latest.x),
                );
                ui.label("Y:");
                ui.colored_label(
                    egui::Color32::from_rgb(100, 255, 100),
                    format!("{:.2}", latest.y),
                );
                ui.label("Z:");
                ui.colored_label(
                    egui::Color32::from_rgb(100, 100, 255),
                    format!("{:.2}", latest.z),
                );
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
                let points_x: PlotPoints = data_x
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
                let points_y: PlotPoints = data_y
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
                let points_z: PlotPoints = data_z
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
        ui.label(format!("Data points: {}", data_x.len()));
    }
}
