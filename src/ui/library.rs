//! Pantalla de biblioteca: barra superior, herramientas, contenido y menús.

use gpui::{
    Anchor, AnyElement, ClickEvent, Context, FontWeight, MouseButton, MouseDownEvent, anchored,
    deferred, div, point, prelude::*, px, svg,
};

use crate::{
    app::{ContextMenuState, NoctuaApp, Sort, View},
    ui::{
        components::{self, ButtonVariant},
        theme,
    },
};

const TOPBAR_HEIGHT: f32 = 104.0;
const TOOLBAR_HEIGHT: f32 = 96.0;
const STATUSBAR_HEIGHT: f32 = 52.0;

impl NoctuaApp {
    fn render_topbar(
        &mut self,
        compact: bool,
        padding: f32,
        viewport_width: f32,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let search_width = if compact {
            (viewport_width * 0.3).min(300.0)
        } else {
            390.0
        };

        let brand = div()
            .flex()
            .items_center()
            .gap(px(12.))
            .when(!compact, |el| el.min_w(px(238.)))
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_center()
                    .size(px(38.))
                    .rounded(px(12.))
                    .bg(theme::surface_active())
                    .child(
                        svg()
                            .path("icons/moon.svg")
                            .size(px(23.))
                            .text_color(theme::accent()),
                    ),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(3.))
                    .child(
                        div()
                            .text_size(px(17.))
                            .font_weight(FontWeight::BOLD)
                            .text_color(theme::text())
                            .child("Noctua"),
                    )
                    .when(!compact, |el| {
                        el.child(
                            div()
                                .text_size(px(10.))
                                .text_color(theme::muted())
                                .child("Tu biblioteca de libros"),
                        )
                    }),
            );

        let search = div()
            .flex()
            .items_center()
            .gap(px(10.))
            .w(px(search_width))
            .h(px(42.))
            .px(px(14.))
            .border_1()
            .border_color(theme::border())
            .rounded(px(8.))
            .bg(theme::surface())
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(|this, _, window, cx| {
                    let handle = this.search.read(cx).handle();
                    window.focus(&handle, cx);
                }),
            )
            .child(
                svg()
                    .path("icons/search.svg")
                    .size(px(18.))
                    .text_color(theme::muted()),
            )
            .child(div().flex_1().min_w_0().child(self.search.clone()))
            .when(!compact, |el| {
                el.child(
                    div()
                        .whitespace_nowrap()
                        .text_size(px(10.))
                        .text_color(theme::faint())
                        .child("Ctrl K"),
                )
            });

        let actions = div()
            .flex()
            .items_center()
            .gap(px(8.))
            .child(
                components::button(
                    "change-folder",
                    "Cambiar carpeta",
                    Some("icons/folder.svg"),
                    ButtonVariant::Secondary,
                )
                .on_click(cx.listener(Self::choose_vault)),
            )
            .child(
                components::button(
                    "open-pdf",
                    "Abrir PDF",
                    Some("icons/external-link.svg"),
                    ButtonVariant::Primary,
                )
                .on_click(cx.listener(Self::choose_pdf)),
            )
            .child(components::icon_button(
                "settings",
                "icons/settings.svg",
                36.0,
            ));

        div()
            .flex()
            .items_center()
            .justify_between()
            .gap(px(if compact { 12. } else { 24. }))
            .h(px(TOPBAR_HEIGHT))
            .px(px(padding))
            .border_b_1()
            .border_color(theme::border())
            .child(brand)
            .child(search)
            .child(actions)
            .into_any_element()
    }

    fn render_toolbar(&mut self, padding: f32, cx: &mut Context<Self>) -> AnyElement {
        let count = format!(
            "{} {}",
            self.filtered.len(),
            if self.filtered.len() == 1 {
                "libro"
            } else {
                "libros"
            }
        );

        let view_toggle = div()
            .flex()
            .gap(px(2.))
            .p(px(3.))
            .border_1()
            .border_color(theme::border())
            .rounded(px(10.))
            .child(
                components::view_button("list-view", "icons/list.svg", self.view == View::List)
                    .on_click(cx.listener(|this, _, _, cx| {
                        this.view = View::List;
                        cx.notify();
                    })),
            )
            .child(
                components::view_button("grid-view", "icons/grid.svg", self.view == View::Grid)
                    .on_click(cx.listener(|this, _, _, cx| {
                        this.view = View::Grid;
                        cx.notify();
                    })),
            );

        div()
            .flex()
            .items_center()
            .justify_between()
            .min_h(px(TOOLBAR_HEIGHT))
            .px(px(padding))
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap(px(14.))
                    .child(
                        div()
                            .text_size(px(20.))
                            .font_weight(FontWeight::BOLD)
                            .text_color(theme::text())
                            .child("Mi biblioteca"),
                    )
                    .child(
                        div()
                            .text_size(px(11.))
                            .text_color(theme::muted())
                            .child(count),
                    ),
            )
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap(px(14.))
                    .child(self.render_sort(cx))
                    .child(view_toggle)
                    .child(
                        components::icon_button("refresh", "icons/refresh.svg", 36.0).on_click(
                            cx.listener(|this, _, _, cx| {
                                this.load_books(cx);
                            }),
                        ),
                    ),
            )
            .into_any_element()
    }

    fn render_sort(&mut self, cx: &mut Context<Self>) -> AnyElement {
        let control = div()
            .id("sort-select")
            .flex()
            .items_center()
            .gap(px(9.))
            .h(px(36.))
            .px(px(11.))
            .border_1()
            .border_color(theme::border())
            .rounded(px(8.))
            .bg(theme::surface())
            .cursor_pointer()
            .on_click(cx.listener(|this, _, _, cx| {
                this.sort_open = !this.sort_open;
                cx.notify();
            }))
            .child(
                svg()
                    .path("icons/sort.svg")
                    .size(px(16.))
                    .text_color(theme::muted()),
            )
            .child(
                div()
                    .text_size(px(11.))
                    .text_color(theme::text())
                    .child(self.sort.label()),
            )
            .child(
                svg()
                    .path("icons/chevron-down.svg")
                    .size(px(14.))
                    .text_color(theme::muted()),
            );

        let options = [Sort::Recent, Sort::Name, Sort::Size];

        div()
            .relative()
            .child(control)
            .when(self.sort_open, |el| {
                el.child(
                    deferred(
                        anchored()
                            .anchor(Anchor::TopLeft)
                            .offset(point(px(0.), px(42.)))
                            .snap_to_window_with_margin(px(8.))
                            .child(
                                components::menu_container()
                                    .id("sort-menu")
                                    .w(px(132.))
                                    .on_mouse_down_out(cx.listener(|this, _, _, cx| {
                                        this.sort_open = false;
                                        cx.notify();
                                    }))
                                    .children(options.into_iter().map(|sort| {
                                        components::menu_item(
                                            format!("sort-{}", sort.label()),
                                            sort.label(),
                                            cx.listener(move |this, _, _, cx| {
                                                this.select_sort(sort, cx);
                                            }),
                                        )
                                    })),
                            ),
                    )
                    .priority(2),
                )
            })
            .into_any_element()
    }

    fn render_content(
        &mut self,
        content_height: f32,
        content_width: f32,
        padding: f32,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let body = if self.filtered.is_empty() {
            div()
                .flex()
                .flex_col()
                .h(px(content_height.max(0.)))
                .child(components::empty_state())
                .into_any_element()
        } else if self.view == View::List {
            self.render_list(cx)
        } else {
            self.render_grid(content_width, cx)
        };

        div()
            .id("library-content")
            .flex_1()
            .min_h_0()
            .px(px(padding))
            .overflow_y_scroll()
            .child(body)
            .into_any_element()
    }

    fn render_list(&mut self, cx: &mut Context<Self>) -> AnyElement {
        let header = div()
            .flex()
            .items_center()
            .gap(px(16.))
            .h(px(38.))
            .px(px(16.))
            .text_size(px(10.))
            .text_color(theme::faint())
            .child(div().flex_1().child("NOMBRE"))
            .child(div().w(px(90.)).child("TAMAÑO"))
            .child(div().w(px(150.)).child("MODIFICADO"))
            .child(div().w(px(76.)));

        let mut rows: Vec<AnyElement> = Vec::with_capacity(self.filtered.len());
        for index in self.filtered.iter().copied() {
            let book = self.books[index].clone();
            let selected = self.selected.as_deref() == Some(book.path.as_str());
            let hovered = self.hovered.as_deref() == Some(book.path.as_str());

            let click_path = book.path.clone();
            let click_name = book.name.clone();
            let button_path = book.path.clone();
            let button_name = book.name.clone();
            let hover_path = book.path.clone();
            let menu_path = book.path.clone();
            let menu_name = book.name.clone();

            let row = div()
                .id(format!("book-row-{index}"))
                .flex()
                .items_center()
                .gap(px(16.))
                .h(px(72.))
                .mb(px(4.))
                .px(px(16.))
                .rounded(px(8.))
                .border_1()
                .border_color(gpui::rgba(0x00000000))
                .when(selected, |el| {
                    el.border_color(theme::selected_border())
                        .bg(theme::surface_active())
                })
                .when(!selected, |el| {
                    el.hover(|style| style.bg(theme::surface_hover()))
                })
                .on_click(cx.listener(move |this, event: &ClickEvent, _window, cx| {
                    this.context = None;
                    if event.click_count() >= 2 {
                        this.open_book(&click_path, &click_name, cx);
                    } else {
                        this.selected = Some(click_path.clone());
                        cx.notify();
                    }
                }))
                .on_mouse_down(
                    MouseButton::Right,
                    cx.listener(move |this, event: &MouseDownEvent, _window, cx| {
                        this.context = Some(ContextMenuState {
                            path: menu_path.clone(),
                            name: menu_name.clone(),
                            position: event.position,
                        });
                        cx.notify();
                    }),
                )
                .on_hover(cx.listener(move |this, is_hovered: &bool, _window, cx| {
                    if *is_hovered {
                        this.hovered = Some(hover_path.clone());
                    } else if this.hovered.as_deref() == Some(hover_path.as_str()) {
                        this.hovered = None;
                    }
                    cx.notify();
                }))
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .items_center()
                        .justify_center()
                        .gap(px(3.))
                        .w(px(34.))
                        .h(px(44.))
                        .rounded(px(5.))
                        .bg(theme::thumb())
                        .child(
                            svg()
                                .path("icons/file-text.svg")
                                .size(px(17.))
                                .text_color(theme::muted()),
                        )
                        .child(
                            div()
                                .text_size(px(7.))
                                .font_weight(FontWeight::BOLD)
                                .text_color(theme::muted())
                                .child("PDF"),
                        ),
                )
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .gap(px(5.))
                        .flex_1()
                        .min_w_0()
                        .child(
                            div()
                                .overflow_hidden()
                                .whitespace_nowrap()
                                .text_ellipsis()
                                .text_size(px(12.))
                                .font_weight(FontWeight::MEDIUM)
                                .text_color(theme::text())
                                .child(book.name.clone()),
                        )
                        .child(
                            div()
                                .overflow_hidden()
                                .whitespace_nowrap()
                                .text_ellipsis()
                                .text_size(px(10.))
                                .text_color(theme::faint())
                                .child(book.relative_path.clone()),
                        ),
                )
                .child(
                    div()
                        .w(px(90.))
                        .text_size(px(11.))
                        .text_color(theme::muted())
                        .child(book.size.clone()),
                )
                .child(
                    div()
                        .w(px(150.))
                        .text_size(px(11.))
                        .text_color(theme::muted())
                        .child(book.modified.clone()),
                )
                .child(
                    div().flex().justify_end().w(px(76.)).child(
                        div()
                            .id(format!("open-{index}"))
                            .flex()
                            .items_center()
                            .justify_center()
                            .h(px(32.))
                            .px(px(12.))
                            .border_1()
                            .border_color(theme::border())
                            .rounded(px(8.))
                            .bg(theme::surface_alt())
                            .text_size(px(12.))
                            .text_color(theme::text())
                            .cursor_pointer()
                            .opacity(if selected || hovered { 1.0 } else { 0.0 })
                            .on_click(cx.listener(move |this, _, _, cx| {
                                cx.stop_propagation();
                                this.context = None;
                                this.open_book(&button_path, &button_name, cx);
                            }))
                            .child("Abrir"),
                    ),
                );

            rows.push(row.into_any_element());
        }

        div().child(header).children(rows).into_any_element()
    }

    fn render_grid(&mut self, content_width: f32, cx: &mut Context<Self>) -> AnyElement {
        let gap = 16.0;
        let columns = (((content_width + gap) / (190.0 + gap)).floor() as usize).max(1);
        // Restamos una fracción de píxel para que el redondeo del layout no
        // haga que la última tarjeta salte de fila.
        let card_width =
            (((content_width - (columns as f32 - 1.0) * gap) / columns as f32) - 0.5).max(120.0);

        let mut cards: Vec<AnyElement> = Vec::with_capacity(self.filtered.len());
        for index in self.filtered.iter().copied() {
            let book = self.books[index].clone();
            let selected = self.selected.as_deref() == Some(book.path.as_str());

            let click_path = book.path.clone();
            let click_name = book.name.clone();
            let button_path = book.path.clone();
            let button_name = book.name.clone();

            let card = div()
                .id(format!("book-card-{index}"))
                .flex()
                .flex_col()
                .w(px(card_width))
                .p(px(12.))
                .border_1()
                .border_color(theme::border())
                .rounded(px(10.))
                .bg(theme::surface())
                .when(selected, |el| {
                    el.border_color(theme::selected_border())
                        .bg(theme::surface_active())
                })
                .when(!selected, |el| {
                    el.hover(|style| {
                        style
                            .border_color(theme::card_hover_border())
                            .bg(theme::card_hover_bg())
                    })
                })
                .on_click(cx.listener(move |this, event: &ClickEvent, _window, cx| {
                    this.context = None;
                    if event.click_count() >= 2 {
                        this.open_book(&click_path, &click_name, cx);
                    } else {
                        this.selected = Some(click_path.clone());
                        cx.notify();
                    }
                }))
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .items_center()
                        .justify_center()
                        .gap(px(6.))
                        .h(px(150.))
                        .mb(px(12.))
                        .rounded(px(7.))
                        .bg(theme::thumb())
                        .child(
                            svg()
                                .path("icons/file-text.svg")
                                .size(px(30.))
                                .text_color(theme::muted()),
                        )
                        .child(
                            div()
                                .text_size(px(10.))
                                .text_color(theme::muted())
                                .child("PDF"),
                        ),
                )
                .child(
                    div()
                        .overflow_hidden()
                        .whitespace_nowrap()
                        .text_ellipsis()
                        .text_size(px(12.))
                        .line_height(px(15.))
                        .font_weight(FontWeight::MEDIUM)
                        .text_color(theme::text())
                        .child(book.name.clone()),
                )
                .child(
                    div()
                        .mt(px(6.))
                        .overflow_hidden()
                        .whitespace_nowrap()
                        .text_ellipsis()
                        .text_size(px(10.))
                        .line_height(px(15.))
                        .text_color(theme::muted())
                        .child(format!("{} · {}", book.size, book.modified)),
                )
                .child(
                    div()
                        .id(format!("card-open-{index}"))
                        .flex()
                        .items_center()
                        .justify_center()
                        .w_full()
                        .h(px(32.))
                        .mt(px(12.))
                        .border_1()
                        .border_color(theme::border())
                        .rounded(px(7.))
                        .bg(theme::surface_alt())
                        .text_size(px(12.))
                        .text_color(theme::text())
                        .cursor_pointer()
                        .on_click(cx.listener(move |this, _, _, cx| {
                            cx.stop_propagation();
                            this.context = None;
                            if this.selected.as_deref() == Some(button_path.as_str()) {
                                this.open_book(&button_path, &button_name, cx);
                            } else {
                                this.selected = Some(button_path.clone());
                                cx.notify();
                            }
                        }))
                        .child(if selected { "Abrir" } else { "Seleccionar" }),
                );

            cards.push(card.into_any_element());
        }

        div()
            .flex()
            .flex_wrap()
            .gap(px(gap))
            .pt(px(4.))
            .pb(px(20.))
            .children(cards)
            .into_any_element()
    }

    fn render_statusbar(&self, padding: f32) -> AnyElement {
        let vault = self
            .vault
            .as_ref()
            .map(|path| path.display().to_string())
            .unwrap_or_default();

        div()
            .flex()
            .items_center()
            .justify_between()
            .min_h(px(STATUSBAR_HEIGHT))
            .px(px(padding))
            .border_t_1()
            .border_color(theme::border())
            .text_size(px(10.))
            .text_color(theme::faint())
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap(px(8.))
                    .min_w_0()
                    .child(
                        svg()
                            .path("icons/folder.svg")
                            .size(px(14.))
                            .text_color(theme::muted()),
                    )
                    .child(
                        div()
                            .min_w_0()
                            .overflow_hidden()
                            .whitespace_nowrap()
                            .text_ellipsis()
                            .child(vault),
                    ),
            )
            .child(div().child(self.status.clone()))
            .into_any_element()
    }

    pub(crate) fn render_overlays(&mut self, cx: &mut Context<Self>) -> Vec<AnyElement> {
        let mut overlays = Vec::new();

        if let Some(menu) = self.context.as_ref() {
            let path = menu.path.clone();
            let name = menu.name.clone();
            let position = menu.position;

            let open_path = path.clone();
            let open_name = name.clone();
            let reveal_path = path.clone();

            overlays.push(
                deferred(
                    anchored()
                        .position(position)
                        .snap_to_window_with_margin(px(8.))
                        .child(
                            components::menu_container()
                                .id("context-menu")
                                .w(px(176.))
                                .on_mouse_down_out(cx.listener(|this, _, _, cx| {
                                    this.context = None;
                                    cx.notify();
                                }))
                                .child(components::menu_item(
                                    "context-open",
                                    "Abrir",
                                    cx.listener(move |this, _, _, cx| {
                                        this.context = None;
                                        this.open_book(&open_path, &open_name, cx);
                                    }),
                                ))
                                .child(components::menu_item(
                                    "context-folder",
                                    "Mostrar en carpeta",
                                    cx.listener(move |this, _, _, cx| {
                                        this.context = None;
                                        this.reveal_book(&reveal_path, cx);
                                    }),
                                )),
                        ),
                )
                .priority(1)
                .into_any_element(),
            );
        }

        overlays
    }

    pub(crate) fn render_library(
        &mut self,
        compact: bool,
        viewport_height: f32,
        viewport_width: f32,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let padding = if compact { 20.0 } else { 32.0 };
        let content_width = (viewport_width - padding * 2.0).max(0.0);
        let content_height = viewport_height - TOPBAR_HEIGHT - TOOLBAR_HEIGHT - STATUSBAR_HEIGHT;

        div()
            .flex()
            .flex_col()
            .size_full()
            .child(self.render_topbar(compact, padding, viewport_width, cx))
            .child(self.render_toolbar(padding, cx))
            .child(self.render_content(content_height, content_width, padding, cx))
            .child(self.render_statusbar(padding))
            .into_any_element()
    }
}
