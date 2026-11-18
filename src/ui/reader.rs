//! Pantalla de lectura de un documento abierto.

use std::sync::Arc;

use gpui::{
    AnyElement, App, ClickEvent, Div, ObjectFit, RenderImage, SharedString, Stateful, Window, div,
    img, prelude::*, px, svg,
};

use crate::ui::{
    components::{ButtonVariant, button},
    theme,
};

const HEADER_HEIGHT: f32 = 64.0;
const FOOTER_HEIGHT: f32 = 44.0;
const PAGE_MARGIN: f32 = 32.0;
const PAGE_MAX_WIDTH: f32 = 720.0;
const A4_RATIO: f32 = 0.707;

/// Datos que necesita la pantalla del lector.
pub struct ReaderContent {
    pub name: SharedString,
    pub path: SharedString,
    pub page: usize,
    pub pages: usize,
    pub image: Option<Arc<RenderImage>>,
    pub error: Option<SharedString>,
}

/// Dibuja el lector con el documento renderizado y su información.
pub fn reader<F, G, H>(
    content: ReaderContent,
    viewport_width: f32,
    viewport_height: f32,
    on_back: F,
    on_prev: G,
    on_next: H,
) -> impl IntoElement
where
    F: Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    G: Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    H: Fn(&ClickEvent, &mut Window, &mut App) + 'static,
{
    let (page_width, page_height) = page_size(viewport_width, viewport_height);
    let page_label = if content.pages > 0 {
        format!("Página {} de {}", content.page, content.pages)
    } else {
        format!("Página {}", content.page)
    };

    let page_body: AnyElement = match content.image.clone() {
        Some(image) => img(image)
            .size_full()
            .object_fit(ObjectFit::Contain)
            .into_any_element(),
        None => {
            let message = content
                .error
                .clone()
                .unwrap_or_else(|| "Aquí aparecerá el documento".into());

            div()
                .flex()
                .flex_col()
                .size_full()
                .items_center()
                .justify_center()
                .gap(px(10.))
                .p(px(24.))
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
                        .child(message),
                )
                .into_any_element()
        }
    };

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
                        .child(content.name),
                ),
        )
        .child(
            div().flex().flex_1().items_center().justify_center().child(
                div()
                    .w(px(page_width))
                    .h(px(page_height))
                    .overflow_hidden()
                    .rounded(px(4.))
                    .bg(theme::page())
                    .border_1()
                    .border_color(theme::border_strong())
                    .child(page_body),
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
                        .child(content.path),
                )
                .child(
                    div()
                        .flex()
                        .items_center()
                        .gap(px(8.))
                        .child(page_button("reader-prev", "‹", on_prev))
                        .child(
                            div()
                                .text_size(px(11.))
                                .text_color(theme::muted())
                                .child(page_label),
                        )
                        .child(page_button("reader-next", "›", on_next)),
                ),
        )
}

fn page_button<F>(id: &'static str, label: &'static str, on_click: F) -> Stateful<Div>
where
    F: Fn(&ClickEvent, &mut Window, &mut App) + 'static,
{
    div()
        .id(id)
        .flex()
        .items_center()
        .justify_center()
        .size(px(26.))
        .rounded(px(6.))
        .border_1()
        .border_color(theme::border())
        .bg(theme::surface_alt())
        .text_size(px(14.))
        .text_color(theme::text())
        .cursor_pointer()
        .hover(|style| style.bg(theme::surface_hover()))
        .on_click(on_click)
        .child(label)
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
