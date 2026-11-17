//! Prueba de diagnóstico de pdfium: abre un PDF, cuenta sus páginas y
//! renderiza la primera a un PNG.
//!
//! Uso:
//!   cargo run --example pdf_probe -- <archivo.pdf> [salida.png]

use std::env;

use pdfium_render::prelude::*;

const PDFIUM_DIR: &str = "vendor/pdfium";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let Some(input) = args.get(1) else {
        eprintln!("uso: cargo run --example pdf_probe -- <archivo.pdf> [salida.png]");
        std::process::exit(2);
    };
    let output = args
        .get(2)
        .cloned()
        .unwrap_or_else(|| "/tmp/opencode/page1.png".to_string());

    let library = Pdfium::pdfium_platform_library_name_at_path(PDFIUM_DIR);
    if !library.exists() {
        return Err(format!(
            "no se encontró {}; ejecuta scripts/fetch-pdfium.sh",
            library.display()
        )
        .into());
    }

    let pdfium = Pdfium::new(Pdfium::bind_to_library(&library)?);

    let document = pdfium.load_pdf_from_file(input, None)?;
    println!("PDF abierto: {} páginas", document.pages().len());

    let page = document.pages().get(0)?;
    println!(
        "Página 1: {:.1} x {:.1} puntos",
        page.width().value,
        page.height().value
    );

    let config = PdfRenderConfig::new().set_target_width(1200);
    let bitmap = page.render_with_config(&config)?;
    let image = bitmap.as_image()?;
    image.save(&output)?;

    println!(
        "Imagen guardada en {} ({}x{})",
        output,
        image.width(),
        image.height()
    );

    Ok(())
}
