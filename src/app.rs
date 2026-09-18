#[derive(Debug, Default)]
pub(crate) struct NoctuaApp;

impl eframe::App for NoctuaApp {
    fn ui(&mut self, ui: &mut eframe::egui::Ui, _frame: &mut eframe::Frame) {
        crate::ui::draw(ui);
    }
}

pub(crate) fn run() -> eframe::Result {
    let options = eframe::NativeOptions::default();

    eframe::run_native(
        "Noctua",
        options,
        Box::new(|_creation_context| Ok(Box::new(NoctuaApp))),
    )
}
