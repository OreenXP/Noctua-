//! Pantalla de bienvenida cuando todavía no hay un vault configurado.

use gpui::{App, ClickEvent, FontWeight, Window, div, prelude::*, px, svg};

use crate::ui::{
    components::{ButtonVariant, button},
    theme,
};

pub fn welcome<F>(on_choose: F) -> impl IntoElement
where
    F: Fn(&ClickEvent, &mut Window, &mut App) + 'static,
{
    div()
        .flex()
        .flex_col()
        .items_center()
        .justify_center()
        .size_full()
        .gap(px(12.))
        .child(
            svg()
                .path("icons/moon.svg")
                .size(px(48.))
                .text_color(theme::accent())
                .mb(px(6.)),
        )
        .child(
            div()
                .text_size(px(26.))
                .font_weight(FontWeight::BOLD)
                .text_color(theme::text())
                .child("Elige tu biblioteca"),
        )
        .child(
            div()
                .mb(px(12.))
                .text_size(px(13.))
                .text_color(theme::muted())
                .child("Selecciona una carpeta donde guardas tus libros PDF."),
        )
        .child(
            button(
                "welcome-choose",
                "Elegir carpeta",
                None,
                ButtonVariant::Primary,
            )
            .on_click(on_choose),
        )
}
