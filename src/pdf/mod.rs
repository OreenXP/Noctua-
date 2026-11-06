//! Utilidades de PDF.
//!
//! De momento solo incluye la detección de archivos PDF. La apertura, lectura
//! y renderizado se añadirán cuando se integre el motor PDF.
use std::path::Path;

pub(crate) fn is_pdf(path: &Path) -> bool {
    let Some(extension) = path.extension() else {
        return false;
    };
    let Some(extension) = extension.to_str() else {
        return false;
    };
    extension.eq_ignore_ascii_case("pdf")
}
