mod gpu;
mod ui;
mod config;
mod utils;

use eframe::egui;
use ui::window::GpuMonitorApp;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0, 600.0])
            .with_title("GPU Monitor")
            .with_resizable(true),
        ..Default::default()
    };
    
    eframe::run_native(
        "GPU Monitor",
        options,
        Box::new(|_cc| {
            // Убрана строка с egui_extras
            Box::new(GpuMonitorApp::new())
        }),
    )
}