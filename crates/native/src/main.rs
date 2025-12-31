use eframe::egui;

use breakout_core::ui;

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([800.0, 600.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Breakout",
        native_options,
        Box::new(|cc| Ok(Box::new(ui::BreakoutApp::new(cc)))),
    )
}
