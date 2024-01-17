use eframe::egui;

use crate::set_font;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct Editor {
    text: String,
}

impl Default for Editor {
    fn default() -> Self {
        Self {
            text: "".to_owned()
        }
    }
}

impl Editor {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        set_font(&cc.egui_ctx);
        if let Some(storage) = cc.storage {
            return eframe::get_value(
                storage, eframe::APP_KEY
            ).unwrap_or_default();
        }
        Default::default()
    }
}

impl eframe::App for Editor {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn auto_save_interval(&self) -> std::time::Duration {
        std::time::Duration::new(1,0)
    }

    fn persist_egui_memory(&self) -> bool {
        true
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::both().show(ui, |ui| {
                ui.horizontal(|ui| {
                    let mut line_numbers: String = "".to_owned();
                    for n in self.text.lines().enumerate() {
                        line_numbers.push_str(&*((n.0 + 1).to_string() + "\n"));
                    }
                    ui.add_sized(
                        egui::vec2(ui.min_size().x, ui.available_height()),
                        egui::TextEdit::multiline(&mut line_numbers)
                            .desired_width(0.0)
                            .interactive(false)
                            .frame(false)
                            .code_editor()
                    );
                    ui.add_sized(
                        egui::vec2(f32::INFINITY, ui.available_height()),
                        egui::TextEdit::multiline(&mut self.text)
                            .code_editor()
                            .desired_width(0.0)
                            .frame(false),
                    );
                });
            })
        });

        egui::TopBottomPanel::bottom("bottom_pannel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                egui::widgets::global_dark_light_mode_switch(ui);
            });
        });
    }
}
