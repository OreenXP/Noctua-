//! Adaptador entre la aplicación y la interfaz creada con Slint.

use slint::ComponentHandle;

slint::include_modules!();

/// Construye y muestra la ventana principal.
pub(crate) fn run() -> Result<(), slint::PlatformError> {
    let window = MainWindow::new()?;

    let weak_window = window.as_weak();
    window.on_abrir_pdf(move || {
        let Some(window) = weak_window.upgrade() else {
            return;
        };

        let Some(path) = rfd::FileDialog::new()
            .add_filter("PDF", &["pdf"])
            .pick_file()
        else {
            return;
        };

        if !crate::pdf::is_pdf(&path) {
            window.set_estado(format!("No es un PDF: {}", path.display()).into());
            return;
        }

        match crate::pdf::open_externally(&path) {
            Ok(()) => window.set_estado(format!("Abierto: {}", path.display()).into()),
            Err(error) => window.set_estado(format!("No se pudo abrir: {error}").into()),
        }
    });

    window.run()
}
