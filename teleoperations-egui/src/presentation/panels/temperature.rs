//! Temperature Panel
//!
//! Displays real-time temperature sensor data with a line plot.

use egui::Ui;
use egui_plot::{Line, Plot, PlotPoints};
use std::collections::VecDeque;

use crate::presentation::view_models::TemperatureViewModel;

pub struct TemperaturePanel;

impl TemperaturePanel {
    pub fn render(ui: &mut Ui, data: &VecDeque<TemperatureViewModel>) {
        ui.heading("🌡️ Temperature Sensor");

        // Current value display
        if let Some(latest) = data.back() {
            ui.horizontal(|ui| {
                ui.label("Current:");
                ui.colored_label(
                    egui::Color32::from_rgb(255, 150, 50),
                    format!("{:.1}°C", latest.value),
                );
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
                let points: PlotPoints = data
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
        ui.label(format!("Data points: {}", data.len()));
    }
}
