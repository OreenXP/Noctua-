pub(crate) fn draw(ui: &mut eframe::egui::Ui) {
    eframe::egui::CentralPanel::default().show(ui, |ui| {
        ui.heading("Noctua");
        ui.label("Empieza el prime.");
    });
}
