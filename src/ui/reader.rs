//! Pantalla de lectura de un documento abierto.
use crate::ui::{
    components::{ButtonVariant, button},
    theme,
};
use gpui::{App, ClickEvent, SharedString, Window, div, prelude::*, px, svg};

const HEADER_HEIGHT: f32 = 64.0;
const FOOTER_HEIGHT: f32 = 44.0;
const PAGE_MARGIN: f32 = 32.0;
const PAGE_MAX_WIDTH: f32 = 720.0;
const A4_RATIO: f32 = 0.707;

/// Dibuja el lector con el nombre y la ruta del documento abierto.
pub fn reader<F>(
    name: SharedString,
    path: SharedString,
    viewport_width: f32,
    viewport_height: f32,
    on_back: F,
) -> impl IntoElement
where
    F: Fn(&ClickEvent, &mut Window, &mut App) + 'static,
{
    let (page_width, page_height) = page_size(viewport_width, viewport_height);

    div()
        .flex()
        .flex_col()
        .size_full()
        .child(
            div()
                .flex()
                .items_center()
                .justify_between()
                .h(px(HEADER_HEIGHT))
                .px(px(24.))
                .border_b_1()
                .border_color(theme::border())
                .child(
                    button("reader-back", "Volver", None, ButtonVariant::Secondary)
                        .on_click(on_back),
                )
                .child(
                    div()
                        .text_size(px(12.))
                        .text_color(theme::muted())
                        .overflow_hidden()
                        .whitespace_nowrap()
                        .text_ellipsis()
                        .child(name),
                ),
        )
        .child(
            div()
                .flex()
                .flex_1()
                .flex_col()
                .items_center()
                .justify_center()
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .items_center()
                        .justify_center()
                        .gap(px(10.))
                        .w(px(page_width))
                        .h(px(page_height))
                        .rounded(px(4.))
                        .bg(theme::page())
                        .border_1()
                        .border_color(theme::border_strong())
                        .child(
                            svg()
                                .path("icons/file-text.svg")
                                .size(px(28.))
                                .text_color(theme::page_text()),
                        )
                        .child(
                            div()
                                .text_size(px(12.))
                                .text_color(theme::page_text())
                                .child("Aquí aparecerá el documento"),
                        ),
                ),
        )
        .child(
            div()
                .flex()
                .items_center()
                .justify_between()
                .h(px(FOOTER_HEIGHT))
                .px(px(24.))
                .border_t_1()
                .border_color(theme::border())
                .child(
                    div()
                        .text_size(px(11.))
                        .overflow_hidden()
                        .whitespace_nowrap()
                        .text_ellipsis()
                        .text_color(theme::faint())
                        .child(path),
                )
                .child(
                    div()
                        .text_size(px(11.))
                        .text_color(theme::muted())
                        .child("Página 1"),
                ),
        )
}

/// Calcula el tamaño de la página para que quepa en la ventana sin deformarse.
fn page_size(viewport_width: f32, viewport_height: f32) -> (f32, f32) {
    let available_width = (viewport_width - PAGE_MARGIN * 2.0).max(0.0);
    let available_height =
        (viewport_height - HEADER_HEIGHT - FOOTER_HEIGHT - PAGE_MARGIN * 2.0).max(0.0);
    let width = (available_height * A4_RATIO)
        .min(available_width)
        .clamp(120.0, PAGE_MAX_WIDTH);
    (width, width / A4_RATIO)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn page_fits_in_a_small_window() {
        let (width, height) = page_size(860.0, 560.0);
        assert!(width <= 860.0 - PAGE_MARGIN * 2.0);
        assert!(
            height <= 560.0 - HEADER_HEIGHT - FOOTER_HEIGHT - PAGE_MARGIN * 2.0 + 0.01,
            "la página no debe desbordar el alto disponible"
        );
    }

    #[test]
    fn page_keeps_the_a4_ratio() {
        let (width, height) = page_size(1200.0, 760.0);
        assert!((width / height - A4_RATIO).abs() < 0.001);
    }

    #[test]
    fn page_does_not_exceed_the_max_width() {
        let (width, _) = page_size(4000.0, 3000.0);
        assert!(width <= PAGE_MAX_WIDTH);
    }
}
