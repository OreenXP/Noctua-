//! Estado raíz de Noctua: navegación entre pantallas y acciones de la app.
//!
//! El dibujado de cada pantalla vive en `ui::library` y `ui::reader`.

use std::path::PathBuf;

use gpui::{
    AnyElement, ClickEvent, Context, Entity, FocusHandle, PathPromptOptions, Pixels, Point, Render,
    Window, actions, div, prelude::*, px,
};

use crate::library::{self, Book};
use crate::ui::{reader, text_input::TextInput, theme, welcome};

actions!(noctua, [FocusSearch, CloseOverlay]);

const COMPACT_WIDTH: f32 = 980.0;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Screen {
    Welcome,
    Library,
    Reader,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum Sort {
    Recent,
    Name,
    Size,
}

impl Sort {
    pub(crate) fn label(self) -> &'static str {
        match self {
            Sort::Recent => "Recientes",
            Sort::Name => "Nombre",
            Sort::Size => "Tamaño",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum View {
    List,
    Grid,
}

struct Document {
    path: String,
    name: String,
}

pub(crate) struct ContextMenuState {
    pub(crate) path: String,
    pub(crate) name: String,
    pub(crate) position: Point<Pixels>,
}

pub struct NoctuaApp {
    screen: Screen,
    document: Option<Document>,
    pub(crate) vault: Option<PathBuf>,
    pub(crate) books: Vec<Book>,
    pub(crate) filtered: Vec<usize>,
    query: String,
    pub(crate) sort: Sort,
    pub(crate) view: View,
    pub(crate) selected: Option<String>,
    pub(crate) hovered: Option<String>,
    pub(crate) context: Option<ContextMenuState>,
    pub(crate) sort_open: bool,
    pub(crate) status: String,
    pub(crate) search: Entity<TextInput>,
    focus_handle: FocusHandle,
}

impl NoctuaApp {
    pub fn new(cx: &mut Context<Self>) -> Self {
        let search = cx.new(|cx| TextInput::new("Buscar libros...", cx));
        let vault = library::configured_vault();

        let mut app = Self {
            screen: if vault.is_some() {
                Screen::Library
            } else {
                Screen::Welcome
            },
            document: None,
            vault,
            books: Vec::new(),
            filtered: Vec::new(),
            query: String::new(),
            sort: Sort::Recent,
            view: View::List,
            selected: None,
            hovered: None,
            context: None,
            sort_open: false,
            status: "Listo".to_string(),
            search,
            focus_handle: cx.focus_handle(),
        };

        cx.observe(&app.search, |this, input, cx| {
            let query = input.read(cx).content().to_string();
            if this.query != query {
                this.query = query;
                this.apply_filters();
                cx.notify();
            }
        })
        .detach();

        if app.vault.is_some() {
            app.load_books(cx);
        }

        app
    }

    pub fn root_focus_handle(&self) -> FocusHandle {
        self.focus_handle.clone()
    }

    fn apply_filters(&mut self) {
        let query = self.query.trim().to_lowercase();
        let mut filtered: Vec<usize> = self
            .books
            .iter()
            .enumerate()
            .filter(|(_, book)| book.name.to_lowercase().contains(&query))
            .map(|(index, _)| index)
            .collect();

        match self.sort {
            Sort::Recent => {
                filtered.sort_by(|a, b| self.books[*b].modified.cmp(&self.books[*a].modified))
            }
            Sort::Name => filtered.sort_by(|a, b| {
                self.books[*a]
                    .name
                    .to_lowercase()
                    .cmp(&self.books[*b].name.to_lowercase())
            }),
            Sort::Size => {
                filtered.sort_by(|a, b| self.books[*b].size_bytes.cmp(&self.books[*a].size_bytes))
            }
        }

        self.filtered = filtered;
    }

    pub(crate) fn load_books(&mut self, cx: &mut Context<Self>) {
        let Some(vault) = self.vault.clone() else {
            return;
        };

        self.status = "Actualizando...".to_string();
        let task = cx.background_spawn(async move { library::load_library(&vault) });

        cx.spawn(async move |this, cx| {
            let result = task.await;
            this.update(cx, |this, cx| {
                match result {
                    Ok(books) => {
                        this.books = books;
                        this.apply_filters();
                        this.status =
                            format!("Vault listo · {} libros encontrados", this.books.len());
                    }
                    Err(error) => {
                        this.status = error;
                    }
                }
                cx.notify();
            })
            .ok();
        })
        .detach();
    }

    pub(crate) fn choose_vault(
        &mut self,
        _: &ClickEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let receiver = cx.prompt_for_paths(PathPromptOptions {
            files: false,
            directories: true,
            multiple: false,
            prompt: Some("Elegir carpeta".into()),
        });

        cx.spawn_in(window, async move |this, cx| {
            let selection = receiver.await;
            this.update_in(cx, |this, _window, cx| match selection {
                Ok(Ok(Some(paths))) if !paths.is_empty() => {
                    let path = paths[0].clone();
                    if let Err(error) = library::save_vault(&path) {
                        this.status = error;
                        cx.notify();
                        return;
                    }
                    this.vault = Some(path);
                    this.screen = Screen::Library;
                    this.selected = None;
                    this.hovered = None;
                    this.load_books(cx);
                }
                Ok(Ok(_)) => {}
                Ok(Err(error)) => {
                    this.status = format!("No se pudo abrir el selector: {error}");
                    cx.notify();
                }
                Err(_) => {}
            })
            .ok();
        })
        .detach();
    }

    pub(crate) fn choose_pdf(
        &mut self,
        _: &ClickEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let receiver = cx.prompt_for_paths(PathPromptOptions {
            files: true,
            directories: false,
            multiple: false,
            prompt: Some("Abrir PDF".into()),
        });

        cx.spawn_in(window, async move |this, cx| {
            let selection = receiver.await;
            this.update_in(cx, |this, _window, cx| match selection {
                Ok(Ok(Some(paths))) if !paths.is_empty() => {
                    let path = paths[0].clone();
                    let name = path
                        .file_name()
                        .and_then(|name| name.to_str())
                        .unwrap_or("PDF")
                        .to_string();
                    cx.open_with_system(&path);
                    this.status = format!("Abierto · {name}");
                    cx.notify();
                }
                Ok(Ok(_)) => {}
                Ok(Err(error)) => {
                    this.status = format!("No se pudo abrir el selector: {error}");
                    cx.notify();
                }
                Err(_) => {}
            })
            .ok();
        })
        .detach();
    }

    pub(crate) fn open_book(&mut self, path: &str, name: &str, cx: &mut Context<Self>) {
        self.document = Some(Document {
            path: path.to_string(),
            name: name.to_string(),
        });
        self.screen = Screen::Reader;
        self.status = format!("Abierto · {name}");
        cx.notify();
    }

    pub(crate) fn reveal_book(&mut self, path: &str, cx: &mut Context<Self>) {
        cx.reveal_path(std::path::Path::new(path));
        cx.notify();
    }

    fn focus_search(&mut self, _: &FocusSearch, window: &mut Window, cx: &mut Context<Self>) {
        let handle = self.search.read(cx).handle();
        window.focus(&handle, cx);
    }

    fn close_overlay(&mut self, _: &CloseOverlay, _: &mut Window, cx: &mut Context<Self>) {
        if self.context.is_some() || self.sort_open {
            self.context = None;
            self.sort_open = false;
            cx.notify();
        }
        if self.screen == Screen::Reader {
            self.back_to_library(cx);
        }
    }

    fn back_to_library(&mut self, cx: &mut Context<Self>) {
        self.screen = Screen::Library;
        self.document = None;
        cx.notify();
    }

    pub(crate) fn select_sort(&mut self, sort: Sort, cx: &mut Context<Self>) {
        self.sort = sort;
        self.sort_open = false;
        self.apply_filters();
        cx.notify();
    }

    fn render_reader(&mut self, cx: &mut Context<Self>) -> AnyElement {
        let Some(document) = &self.document else {
            return div().into_any_element();
        };

        reader::reader(
            document.name.clone().into(),
            document.path.clone().into(),
            cx.listener(|this, _, _, cx| this.back_to_library(cx)),
        )
        .into_any_element()
    }
}

impl Render for NoctuaApp {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let viewport = window.viewport_size();
        let viewport_width = f32::from(viewport.width);
        let viewport_height = f32::from(viewport.height);
        let compact = viewport_width < COMPACT_WIDTH;

        let screen = match self.screen {
            Screen::Welcome => welcome::welcome(cx.listener(Self::choose_vault)).into_any_element(),
            Screen::Library => self.render_library(compact, viewport_height, viewport_width, cx),
            Screen::Reader => self.render_reader(cx),
        };

        div()
            .flex()
            .flex_col()
            .size_full()
            .bg(theme::bg())
            .text_size(px(13.))
            .text_color(theme::text())
            .track_focus(&self.focus_handle)
            .on_action(cx.listener(Self::focus_search))
            .on_action(cx.listener(Self::close_overlay))
            .child(screen)
            .children(self.render_overlays(cx))
    }
}
