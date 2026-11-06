#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod library;
mod pdf;
mod ui;

use app::{CloseOverlay, FocusSearch, NoctuaApp};
use gpui::{App, AppContext as _, Bounds, KeyBinding, WindowBounds, WindowOptions, px, size};
use gpui_platform::application;
use ui::assets::Assets;
use ui::text_input::{
    Backspace, Copy, Cut, Delete, End, Home, Left, Paste, Right, SelectAll, SelectLeft, SelectRight,
};

fn bind_keys(cx: &mut App) {
    let text = Some("TextInput");

    cx.bind_keys([
        KeyBinding::new("ctrl-k", FocusSearch, None),
        KeyBinding::new("escape", CloseOverlay, None),
        KeyBinding::new("backspace", Backspace, text),
        KeyBinding::new("delete", Delete, text),
        KeyBinding::new("left", Left, text),
        KeyBinding::new("right", Right, text),
        KeyBinding::new("shift-left", SelectLeft, text),
        KeyBinding::new("shift-right", SelectRight, text),
        KeyBinding::new("ctrl-a", SelectAll, text),
        KeyBinding::new("cmd-a", SelectAll, text),
        KeyBinding::new("home", Home, text),
        KeyBinding::new("end", End, text),
        KeyBinding::new("ctrl-v", Paste, text),
        KeyBinding::new("cmd-v", Paste, text),
        KeyBinding::new("ctrl-c", Copy, text),
        KeyBinding::new("cmd-c", Copy, text),
        KeyBinding::new("ctrl-x", Cut, text),
        KeyBinding::new("cmd-x", Cut, text),
    ]);
}

fn main() {
    application().with_assets(Assets).run(|cx: &mut App| {
        bind_keys(cx);

        let bounds = Bounds::centered(None, size(px(1200.0), px(760.0)), cx);
        let window = cx
            .open_window(
                WindowOptions {
                    window_bounds: Some(WindowBounds::Windowed(bounds)),
                    window_min_size: Some(size(px(860.0), px(560.0))),
                    titlebar: Some(gpui::TitlebarOptions {
                        title: Some("Noctua".into()),
                        ..Default::default()
                    }),
                    app_id: Some("com.oreen.noctua".into()),
                    ..Default::default()
                },
                |window, cx| {
                    let view = cx.new(NoctuaApp::new);
                    let handle = view.read(cx).root_focus_handle();
                    window.focus(&handle, cx);
                    view
                },
            )
            .expect("no se pudo abrir la ventana de Noctua");

        window
            .update(cx, |_, window, _| window.activate_window())
            .ok();
        cx.activate(true);
    });
}
