//! Pantalla de lectura de un documento abierto.

use gpui::{App, ClickEvent, SharedString, Window, div, prelude::*, px};

use crate::ui::{
    components::{ButtonVariant, button},
    theme,
};

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
                .h(px(64.))
                .px(px(24.))
                .border_b_1()
                .border_color(theme::border())
                .child(
                    button("reader-back", "Volver", None, ButtonVariant::Secondary)
                        .on_click(on_back),
                ),
        )
        .child(
            div()
                .flex()
                .flex_1()
                .flex_col()
                .items_center()
                .justify_center()
                .gap(px(6.))
                .child(
                    div()
                        .text_size(px(14.))
                        .text_color(theme::text())
                        .child(name),
                )
                .child(
                    div()
                        .text_size(px(11.))
                        .text_color(theme::faint())
                        .child(path),
                ),
        )
}
