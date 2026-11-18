//! Utilidades de PDF.
//!
//! De momento solo incluye la detección de archivos PDF. La apertura, lectura
//! y renderizado se añadirán cuando se integre el motor PDF.
use std::path::Path;

/// Indica si una página es mayoritariamente clara (papel con texto).
///
/// Se usa para no invertir portadas o páginas que son una imagen oscura:
/// en esas, el modo oscuro haría más mal que bien.
pub(crate) fn is_mostly_light(rgba: &[u8]) -> bool {
    let mut light = 0usize;
    let mut total = 0usize;

    for pixel in rgba.as_chunks::<4>().0 {
        let luminance = 0.299 * pixel[0] as f32 + 0.587 * pixel[1] as f32 + 0.114 * pixel[2] as f32;
        if luminance > 200.0 {
            light += 1;
        }
        total += 1;
    }

    total > 0 && light * 100 / total > 70
}

/// Invierte la luminosidad de una imagen RGBA para leer sobre fondo oscuro.
///
/// El papel blanco se convierte en un gris muy oscuro y la tinta negra en
/// blanco. Los píxeles con color (portadas, fotos, gráficos) se dejan casi
/// intactos para que no parezcan un negativo: la inversión pierde fuerza a
/// medida que aumenta la saturación.
pub(crate) fn darken_rgba(rgba: &mut [u8]) {
    for pixel in rgba.as_chunks_mut::<4>().0 {
        let red = pixel[0] as f32;
        let green = pixel[1] as f32;
        let blue = pixel[2] as f32;

        let max = red.max(green).max(blue);
        let min = red.min(green).min(blue);
        let saturation = if max > 0.0 { (max - min) / max } else { 0.0 };

        let luminance = 0.299 * red + 0.587 * green + 0.114 * blue;
        let inverted = 24.0 + (255.0 - luminance) * (231.0 / 255.0);
        let strength = (1.0 - saturation / 0.25).clamp(0.0, 1.0);
        let target = luminance + (inverted - luminance) * strength;
        let delta = target - luminance;

        pixel[0] = (red + delta).clamp(0.0, 255.0) as u8;
        pixel[1] = (green + delta).clamp(0.0, 255.0) as u8;
        pixel[2] = (blue + delta).clamp(0.0, 255.0) as u8;
    }
}

pub(crate) fn is_pdf(path: &Path) -> bool {
    let Some(extension) = path.extension() else {
        return false;
    };
    let Some(extension) = extension.to_str() else {
        return false;
    };
    extension.eq_ignore_ascii_case("pdf")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn darken_turns_white_into_dark() {
        let mut pixel = [255, 255, 255, 255];
        darken_rgba(&mut pixel);
        assert!(pixel[0] < 40 && pixel[1] < 40 && pixel[2] < 40);
        assert_eq!(pixel[3], 255);
    }

    #[test]
    fn darken_turns_black_into_light() {
        let mut pixel = [0, 0, 0, 255];
        darken_rgba(&mut pixel);
        assert!(pixel[0] > 240 && pixel[1] > 240 && pixel[2] > 240);
        assert_eq!(pixel[3], 255);
    }

    #[test]
    fn darken_keeps_colored_pixels() {
        let mut pixel = [200, 50, 50, 255];
        darken_rgba(&mut pixel);
        assert_eq!(pixel, [200, 50, 50, 255]);
    }

    #[test]
    fn detects_light_and_dark_pages() {
        assert!(is_mostly_light(&[255; 400]));
        assert!(!is_mostly_light(&[20; 400]));
    }
}
