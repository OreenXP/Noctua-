//! Adaptador entre la aplicación y la interfaz creada con Slint.

use slint::ComponentHandle;

slint::include_modules!();

/// Construye y muestra la ventana principal.
pub(crate) fn run() -> Result<(), slint::PlatformError> {
    let window = MainWindow::new()?;
    window.run()
}
