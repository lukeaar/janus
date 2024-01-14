use eframe::egui;

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "janus",
        options,
        Box::new(|_cc| Box::new(Editor::default())),
    );
}

#[derive(Default)]
struct Editor {
    language: String,
    text: String,
}

impl eframe::App for Editor {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
                egui::ScrollArea::both().show(ui, |ui| {
                    ui.add_sized(
                        egui::vec2(f32::INFINITY, ui.available_height()),
                        egui::TextEdit::multiline(&mut self.text)
                            .font(egui::TextStyle::Monospace)
                            .code_editor()
                            .lock_focus(true)
                            .desired_width(f32::INFINITY),
                    );
                });
        });
    }
}
