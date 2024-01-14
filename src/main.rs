use eframe::egui;

fn main() {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0, 300.0])
            .with_min_inner_size([300.0, 220.0])
            .with_icon(
                eframe::icon_data::from_png_bytes(&include_bytes!("../assets/icon.png")[..])
                    .unwrap(),
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
#[derive(Default)]
struct Editor {
    text: String,
}

impl Editor {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {

        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Self::default()
    }
}

impl eframe::App for Editor {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ctx.set_pixels_per_point(10.0);
            ui.heading("HEADING");
            ui.horizontal(|ui| {
                ui.label("Write something: ");
                ui.text_edit_singleline(&mut "test".to_string());
            });
            /* ßß */
        });

        egui::TopBottomPanel::bottom("bottom_pannel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                egui::widgets::global_dark_light_mode_buttons(ui);
            });
        });
    }
}
