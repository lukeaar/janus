use epi::egui;

fn main() {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            //.with_inner_size([400.0, 300.0])
            //.with_min_inner_size([300.0, 220.0])
            .with_icon(
                eframe::icon_data::from_png_bytes(
                    &include_bytes!("../assets/icon.png")[..]
                ).unwrap(),
            ),
        
        ..Default::default()
    };
    
    let _ = eframe::run_native(
        "janus",
        options,
        Box::new(|cc| Box::new(Editor::new(cc))),
    );
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
struct Editor {
    text: String,
}

impl Default for Editor {
    fn default() -> Self {
        Self {
            text: "".to_string().to_owned(),
        }
    }
}

impl Editor {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        if let Some(storage) = cc.storage {
            return eframe::get_value(
                storage, eframe::APP_KEY
            ).unwrap_or_default();
        }
        Self::default()
    }
}

impl epi::App for Editor {
    fn name(&self) -> &str {
        "janus"
    }

    fn setup(
        &mut self,
        ctx: &egui::Context,
        _frame: &epi::Frame,
        _storage: Option<&dyn epi::Storage>,
    ) {
        let mut fonts = egui::FontDefinitions::default();
        fonts.font_data.insert(
            "JetBrainsMono Nerd Font Mono".to_owned(),
            egui::FontData::from_static(include_bytes!("../assets/JetBrainsMonoNerdFontMono-Regular.ttf"))
        );
        fonts
            .families
            .entry(egui::FontFamily::Monospace)
            .or_default()
            .insert(0, "JetBrainsMono Nerd Font Mono".to_owned());
        ctx.set_fonts(fonts);
    }

    fn save(&mut self, storage: &mut dyn epi::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn auto_save_interval(&self) -> std::time::Duration {
        std::time::Duration::new(1,0)
    }

    fn persist_egui_memory(&self) -> bool {
        true
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &epi::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::both().show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.vertical(|ui| {
                        for n in self.text.lines().enumerate(){
                            ui.label(
                                egui::RichText::new((n.0 + 1).to_string())
                            );
                        }
                    });
                    ui.add_sized(
                        egui::vec2(f32::INFINITY, ui.available_height()),
                        egui::TextEdit::multiline(&mut self.text)
                            .code_editor()
                            .desired_width(0.0)
                            .frame(false),
                    );
                });
            });
        });

        egui::TopBottomPanel::bottom("bottom_pannel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                egui::widgets::global_dark_light_mode_switch(ui);
            });
        });
    }
}
