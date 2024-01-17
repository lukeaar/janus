#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui;
use janus::editor::Editor;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_icon(
                eframe::icon_data::from_png_bytes(
                    &include_bytes!("../assets/icon.png")[..]
                ).unwrap(),
            ),
        ..Default::default()
    };

    eframe::run_native(
        "janus",
        options,
        Box::new(|cc| Box::new(Editor::new(cc))),
    )
}