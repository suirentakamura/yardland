use egui::{Align2, FontId};

/// Memory map widget
///
/// ```text
///   00 DE AD BE EF 00
///   00 DE AD BE EF 00
///   00 DE AD BE EF 00
///   00 DE AD BE EF 00
/// ```
///
/// ## Example:
/// ```ignore
/// memory_map_ui(ui, &my_memory);
/// ```
pub fn memory_map_ui(ui: &mut egui::Ui, memory: &[u8]) -> egui::Response {
    let desired_size = ui.spacing().default_area_size;

    let (rect, mut response) = ui.allocate_exact_size(desired_size, egui::Sense::drag());

    response.widget_info(|| {
        egui::WidgetInfo::new(egui::WidgetType::Other)
    });

    if ui.is_rect_visible(rect) {
        let visuals = ui.style().noninteractive();

        let rect = rect.expand(visuals.expansion);

        let mut i = 0.0;
        let mut j = 0.0;

        ui.painter()
            .text();

        for byte in memory.iter() {
            let pos = rect.left_top() + egui::vec2(32.0 + i * 32.0, j * 32.0);

            ui.painter()
                .text(pos, Align2::CENTER_CENTER, format!("{:02X}", byte), FontId::monospace(12.0), visuals.text_color());

            i += 1.0;
            if i >= 8.0 {
                i = 0.0;
                j += 1.0;
            }
        }
    }

    response
}
