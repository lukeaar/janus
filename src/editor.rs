use eframe::egui;
use crate::set_font;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct Editor {
    text: String,
    //theme: Theme,
    //font_size: f32
}

impl Default for Editor {
    fn default() -> Self {
        Self {
            text: "".to_owned(),
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
        //self.text = self.text.replace("\t", "    ");

        let mut line_numbers: String = "".to_owned();
        let mut longest_line_length: f32 = 0.0;
        let mut last_line_number: usize = 0;
        for line in self.text.lines().enumerate() {
            line_numbers.push_str(&*((line.0 + 1).to_string() + "\n"));
            last_line_number = line.0 + 1;
            if (line.1.len() as f32) * 8.0 > longest_line_length {
                longest_line_length = (line.1.len() as f32) * 8.0;
            }
        }
        if self.text.len() == 0 || self.text.ends_with('\n') {
            last_line_number = last_line_number + 1;
            line_numbers.push_str(&*(last_line_number.to_string()));
        }

        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.add(egui::Label::new("Ln ".to_string() + &*(last_line_number.to_string())));
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            if ui.available_width() - 32.0 > longest_line_length {
                longest_line_length = ui.available_width() - 32.0
            }
            ui.with_layout(egui::Layout::left_to_right(egui::Align::Center).with_cross_justify(true), |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.add_sized(
                            egui::vec2(ui.min_size().x, ui.available_height()),
                            egui::TextEdit::multiline(&mut line_numbers)
                                .desired_width(0.0)
                                .interactive(false)
                                .frame(false)
                                .code_editor()
                        );
                        egui::ScrollArea::horizontal().show(ui, |ui| {
                            let text_box = ui.add_sized(
                                egui::vec2(longest_line_length, ui.available_height()),
                                egui::TextEdit::multiline(&mut self.text)
                                    .code_editor()
                                    .desired_width(f32::INFINITY)
                                    .frame(false),
                            );
                        });
                    });
                })
            });
        });
    }
}
