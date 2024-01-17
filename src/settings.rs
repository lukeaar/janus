use eframe::egui;

pub fn set_font(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();
    
    fonts.font_data.insert(
        "JetBrainsMono Nerd Font Mono".to_owned(),
        egui::FontData::from_static(include_bytes!(
            "../assets/JetBrainsMonoNerdFontMono-Regular.ttf"
        )),
    );

    fonts
        .families.entry(egui::FontFamily::Monospace)
        .or_default()
        .insert(0, "JetBrainsMono Nerd Font Mono".to_owned());

    ctx.set_fonts(fonts);
}