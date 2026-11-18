//! Estado raíz de Noctua: navegación entre pantallas y acciones de la app.
//!
//! El dibujado de cada pantalla vive en `ui::library` y `ui::reader`.

use std::path::PathBuf;
use std::sync::Arc;

use gpui::{
    AnyElement, ClickEvent, Context, Entity, FocusHandle, PathPromptOptions, Pixels, Point, Render,
    RenderImage, Window, actions, div, prelude::*, px,
};
use pdfium_bundled::pdfium_render::prelude::*;

use crate::library::{self, Book};
use crate::pdf;
use crate::ui::{reader, text_input::TextInput, theme, welcome};

actions!(noctua, [FocusSearch, CloseOverlay, NextPage, PrevPage]);

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
    page: usize,
    pages: usize,
}

pub(crate) struct ContextMenuState {
    pub(crate) path: String,
    pub(crate) name: String,
    pub(crate) position: Point<Pixels>,
}

pub struct NoctuaApp {
    screen: Screen,
    document: Option<Document>,
    pdfium: Option<Pdfium>,
    page_image: Option<Arc<RenderImage>>,
    page_error: Option<String>,
    dark_page: bool,
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
            pdfium: None,
            page_image: None,
            page_error: None,
            dark_page: true,
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

    pub(crate) fn open_book(
        &mut self,
        path: &str,
        name: &str,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.document = Some(Document {
            path: path.to_string(),
            name: name.to_string(),
            page: 1,
            pages: 0,
        });
        self.page_image = None;
        self.page_error = None;
        self.screen = Screen::Reader;
        window.focus(&self.focus_handle, cx);
        self.status = format!("Abriendo · {name}");
        self.load_page(0);
        cx.notify();
    }

    fn load_page(&mut self, page_index: usize) {
        let Some(path) = self.document.as_ref().map(|document| document.path.clone()) else {
            return;
        };

        match self.render_page(&path, page_index) {
            Ok((image, pages)) => {
                if let Some(document) = self.document.as_mut() {
                    document.page = page_index + 1;
                    document.pages = pages;
                }
                self.page_image = Some(Arc::new(image));
                self.page_error = None;
                self.status = format!("Página {} de {pages}", page_index + 1);
            }
            Err(error) => {
                self.page_image = None;
                self.page_error = Some(error.clone());
                self.status = error;
            }
        }
    }

    fn render_page(
        &mut self,
        path: &str,
        page_index: usize,
    ) -> Result<(RenderImage, usize), String> {
        if self.pdfium.is_none() {
            self.pdfium = Some(
                pdfium_bundled::bind_pdfium_silent()
                    .map_err(|error| format!("No se pudo cargar pdfium: {error}"))?,
            );
        }

        let pdfium = self.pdfium.as_ref().expect("pdfium inicializado");
        let document = pdfium
            .load_pdf_from_file(path, None)
            .map_err(|error| format!("No se pudo abrir el PDF: {error}"))?;
        let pages = document.pages().len() as usize;
        let page = document
            .pages()
            .get(page_index as i32)
            .map_err(|error| format!("No se pudo leer la página: {error}"))?;

        let config = PdfRenderConfig::new().set_target_width(1600);
        let bitmap = page
            .render_with_config(&config)
            .map_err(|error| format!("No se pudo renderizar la página: {error}"))?;

        let width = bitmap.width() as u32;
        let height = bitmap.height() as u32;
        let mut rgba = bitmap.as_rgba_bytes();
        if self.dark_page && pdf::is_mostly_light(&rgba) {
            pdf::darken_rgba(&mut rgba);
        }

        let buffer = image::RgbaImage::from_raw(width, height, rgba)
            .ok_or_else(|| "El bitmap renderizado tiene un tamaño inesperado".to_string())?;

        Ok((RenderImage::new(vec![image::Frame::new(buffer)]), pages))
    }

    pub(crate) fn reveal_book(&mut self, path: &str, cx: &mut Context<Self>) {
        cx.reveal_path(std::path::Path::new(path));
        cx.notify();
    }

    pub(crate) fn open_external(&mut self, path: &str, name: &str, cx: &mut Context<Self>) {
        cx.open_with_system(std::path::Path::new(path));
        self.status = format!("Abierto con el visor del sistema · {name}");
        cx.notify();
    }

    fn focus_search(&mut self, _: &FocusSearch, window: &mut Window, cx: &mut Context<Self>) {
        let handle = self.search.read(cx).handle();
        window.focus(&handle, cx);
    }

    fn close_overlay(&mut self, _: &CloseOverlay, window: &mut Window, cx: &mut Context<Self>) {
        if self.context.is_some() || self.sort_open {
            self.context = None;
            self.sort_open = false;
            cx.notify();
        }
        if self.screen == Screen::Reader {
            self.back_to_library(window, cx);
        }
    }

    fn back_to_library(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.screen = Screen::Library;
        self.document = None;
        self.page_image = None;
        self.page_error = None;

        let handle = self.search.read(cx).handle();
        window.focus(&handle, cx);
        cx.notify();
    }

    pub(crate) fn select_sort(&mut self, sort: Sort, cx: &mut Context<Self>) {
        self.sort = sort;
        self.sort_open = false;
        self.apply_filters();
        cx.notify();
    }

    fn next_page(&mut self, _: &NextPage, _: &mut Window, cx: &mut Context<Self>) {
        self.change_page(1, cx);
    }

    fn prev_page(&mut self, _: &PrevPage, _: &mut Window, cx: &mut Context<Self>) {
        self.change_page(-1, cx);
    }

    fn change_page(&mut self, delta: isize, cx: &mut Context<Self>) {
        if self.screen != Screen::Reader {
            return;
        }
        let Some(document) = self.document.as_ref() else {
            return;
        };

        let current = document.page.saturating_sub(1) as isize;
        let last = document.pages.saturating_sub(1) as isize;
        let target = (current + delta).clamp(0, last);
        if target == current {
            return;
        }

        self.load_page(target as usize);
        cx.notify();
    }

    fn toggle_dark_page(&mut self, _: &ClickEvent, _: &mut Window, cx: &mut Context<Self>) {
        self.dark_page = !self.dark_page;

        if let Some(page) = self
            .document
            .as_ref()
            .map(|document| document.page.saturating_sub(1))
        {
            self.load_page(page);
        }

        cx.notify();
    }

    fn render_reader(
        &mut self,
        viewport_width: f32,
        viewport_height: f32,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let Some(document) = &self.document else {
            return div().into_any_element();
        };

        let content = reader::ReaderContent {
            name: document.name.clone().into(),
            path: document.path.clone().into(),
            page: document.page,
            pages: document.pages,
            image: self.page_image.clone(),
            error: self.page_error.clone().map(Into::into),
            dark_page: self.dark_page,
        };

        reader::reader(
            content,
            viewport_width,
            viewport_height,
            cx.listener(|this, _, window, cx| this.back_to_library(window, cx)),
            cx.listener(|this, _, _, cx| this.change_page(-1, cx)),
            cx.listener(|this, _, _, cx| this.change_page(1, cx)),
            cx.listener(Self::toggle_dark_page),
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
            Screen::Reader => self.render_reader(viewport_width, viewport_height, cx),
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
            .on_action(cx.listener(Self::next_page))
            .on_action(cx.listener(Self::prev_page))
            .child(screen)
            .children(self.render_overlays(cx))
    }
}
