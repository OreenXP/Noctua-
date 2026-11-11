//! Pantalla de lectura de un documento abierto.
use crate::ui::{
    components::{ButtonVariant, button},
    theme,
};
use gpui::{App, ClickEvent, SharedString, Window, div, prelude::*, px, svg};

/// Dibuja el lector con el nombre y la ruta del documento abierto.
pub fn reader<F>(name: SharedString, path: SharedString, on_back: F) -> impl IntoElement
where
    F: Fn(&ClickEvent, &mut Window, &mut App) + 'static,
{
    div()
        .flex()
        .flex_col()
        .size_full()
        .child(
            div()
                .flex()
                .items_center()
                .justify_between()
                .h(px(64.))
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
                        .w(px(280.))
                        .aspect_ratio(0.707)
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
                .h(px(44.))
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
