//! Occupancy Panel
//!
//! Displays room occupancy state with visual indicators.

use egui::Ui;

use crate::presentation::view_models::OccupancyViewModel;

pub struct OccupancyPanel;

impl OccupancyPanel {
    pub fn render(ui: &mut Ui, data: &OccupancyViewModel) {
        ui.heading("🚪 Room Occupancy");

        ui.add_space(10.0);

        // Status display
        let status_text = data.get_status_text();
        let status_color = data.get_status_color();

        ui.horizontal(|ui| {
            ui.label("Status:");
            ui.colored_label(status_color, status_text);
        });

        ui.add_space(10.0);

        // People count
        ui.horizontal(|ui| {
            ui.label("People Count:");
            ui.colored_label(egui::Color32::WHITE, format!("{}", data.count));
        });

        ui.add_space(10.0);

        // Last motion
        if data.last_motion_time > 0 {
            let elapsed =
                (chrono::Utc::now().timestamp_millis() - data.last_motion_time) / 1000;
            ui.horizontal(|ui| {
                ui.label("Last Motion:");
                ui.colored_label(egui::Color32::LIGHT_GRAY, format!("{}s ago", elapsed));
            });
        }

        ui.add_space(20.0);

        // Visual indicator
        Self::render_status_indicator(ui, data);
    }

    fn render_status_indicator(ui: &mut Ui, data: &OccupancyViewModel) {
        let rect_size = egui::vec2(200.0, 100.0);
        let (rect, _response) = ui.allocate_exact_size(rect_size, egui::Sense::hover());

        if ui.is_rect_visible(rect) {
            let painter = ui.painter();
            let status_color = data.get_status_color();

            // Draw background
            painter.rect_filled(rect, 5.0, status_color.linear_multiply(0.3));
            painter.rect_stroke(rect, 5.0, egui::Stroke::new(2.0, status_color));

            // Draw icon
            let center = rect.center();
            let icon = if data.occupied { "👥" } else { "🚫" };

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
