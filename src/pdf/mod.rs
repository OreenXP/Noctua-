//! Apertura, lectura y renderizado de documentos PDF.
//!
//! El módulo se implementará cuando se integre el motor PDF. Permanece vacío
//! para establecer desde el inicio su frontera dentro de la aplicación.
use std::path::Path;

fn main(){

}

pub(crate) fn is_pdf(path: &Path) -> bool {
    let Some(extension) = path.extension() else {
        return false;
    };
    let Some(extension) = path.to_str() else {
        return false
    };
    extension.eq_ignore_ascii_case("pdf")
}