//! Componentes visuales reutilizables de Noctua.

use gpui::{
    App, ClickEvent, Div, ElementId, SharedString, Stateful, Window, div, prelude::*, px, svg,
};

use crate::ui::theme;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ButtonVariant {
    Primary,
    Secondary,
}

/// Botón de la barra superior, con icono opcional a la izquierda.
pub fn button(
    id: impl Into<ElementId>,
    label: impl Into<SharedString>,
    icon: Option<&'static str>,
    variant: ButtonVariant,
) -> Stateful<Div> {
    let base = div()
        .id(id)
        .flex()
        .items_center()
        .justify_center()
        .gap(px(8.))
        .h(px(40.))
        .px(px(14.))
        .rounded(px(8.))
        .border_1()
        .text_size(px(12.))
        .whitespace_nowrap()
        .cursor_pointer();

    let styled = match variant {
        ButtonVariant::Primary => base
            .bg(theme::accent())
            .border_color(theme::accent())
            .text_color(theme::accent_text())
            .hover(|style| {
                style
                    .bg(theme::accent_hover())
                    .border_color(theme::accent_hover())
            }),
        ButtonVariant::Secondary => base
            .bg(theme::surface_alt())
            .border_color(theme::border())
            .text_color(theme::text())
            .hover(|style| style.bg(theme::button_hover())),
    };

    match icon {
        Some(path) => styled
            .child(
                svg()
                    .path(path)
                    .w(px(16.))
                    .h(px(16.))
                    .text_color(match variant {
                        ButtonVariant::Primary => theme::accent_text(),
                        ButtonVariant::Secondary => theme::muted(),
                    }),
            )
            .child(label.into()),
        None => styled.child(label.into()),
    }
}

/// Botón cuadrado sin fondo para iconos.
pub fn icon_button(id: impl Into<ElementId>, icon: &'static str, size: f32) -> Stateful<Div> {
    div()
        .id(id)
        .flex()
        .items_center()
        .justify_center()
        .size(px(size))
        .rounded(px(8.))
        .cursor_pointer()
        .child(
            svg()
                .path(icon)
                .size(px(size / 2.))
                .text_color(theme::muted()),
        )
}

/// Botón del selector lista/cuadrícula.
pub fn view_button(id: impl Into<ElementId>, icon: &'static str, active: bool) -> Stateful<Div> {
    let base = div()
        .id(id)
        .flex()
        .items_center()
        .justify_center()
        .size(px(32.))
        .rounded(px(8.))
        .cursor_pointer();

    let base = if active {
        base.bg(theme::surface_active())
    } else {
        base
    };

    base.child(
        svg()
            .path(icon)
            .size(px(18.))
            .text_color(if icon.ends_with("list.svg") {
                theme::accent()
            } else {
                theme::muted()
            }),
    )
}

/// Fila de menú contextual o desplegable.
pub fn menu_item<F>(id: impl Into<ElementId>, label: &'static str, on_click: F) -> Stateful<Div>
where
    F: Fn(&ClickEvent, &mut Window, &mut App) + 'static,
{
    div()
        .id(id)
        .flex()
        .items_center()
        .w_full()
        .h(px(32.))
        .px(px(10.))
        .rounded(px(6.))
        .text_size(px(12.))
        .text_color(theme::text())
        .cursor_pointer()
        .hover(|style| style.bg(theme::surface_active()))
        .on_click(on_click)
        .child(label)
}

/// Contenedor visual de un menú flotante.
pub fn menu_container() -> Div {
    div()
        .occlude()
        .p(px(5.))
        .border_1()
        .border_color(theme::border_strong())
        .rounded(px(9.))
        .bg(theme::surface_alt())
        .shadow_lg()
}

/// Estado vacío de la biblioteca, igual al de la interfaz anterior.
pub fn empty_state() -> Div {
    div()
        .flex()
        .flex_col()
        .items_center()
        .justify_center()
        .h_full()
        .child(
            svg()
                .path("icons/file-text.svg")
                .size(px(30.))
                .text_color(theme::muted())
                .opacity(0.7),
        )
        .child(
            div()
                .mt(px(14.))
                .text_size(px(15.))
                .font_weight(gpui::FontWeight::BOLD)
                .text_color(theme::text())
                .child("No hay libros todavía"),
        )
        .child(
            div()
                .mt(px(6.))
                .text_size(px(12.))
                .text_color(theme::muted())
                .child("Agrega archivos PDF a tu vault para verlos aquí."),
        )
}
