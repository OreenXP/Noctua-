//! Tokens visuales de Noctua, tomados de la interfaz web anterior.

use gpui::{Rgba, rgb};

pub fn bg() -> Rgba {
    rgb(0x111216)
}

pub fn surface() -> Rgba {
    rgb(0x17181e)
}

pub fn surface_alt() -> Rgba {
    rgb(0x1d1f27)
}

pub fn surface_hover() -> Rgba {
    rgb(0x191b23)
}

pub fn surface_active() -> Rgba {
    rgb(0x252a40)
}

pub fn thumb() -> Rgba {
    rgb(0x252936)
}

pub fn border() -> Rgba {
    rgb(0x2b2d37)
}

pub fn border_strong() -> Rgba {
    rgb(0x343747)
}

pub fn text() -> Rgba {
    rgb(0xececf1)
}

pub fn muted() -> Rgba {
    rgb(0xa0a3b3)
}

pub fn faint() -> Rgba {
    rgb(0x777c8e)
}

pub fn accent() -> Rgba {
    rgb(0xadb9fa)
}

pub fn accent_hover() -> Rgba {
    rgb(0xc9d7ff)
}

pub fn accent_text() -> Rgba {
    rgb(0x141a30)
}

pub fn accent_soft() -> Rgba {
    gpui::rgba(0xadb9fa40)
}

pub fn selected_border() -> Rgba {
    rgb(0x3c4669)
}

pub fn card_hover_border() -> Rgba {
    rgb(0x4b557e)
}

pub fn card_hover_bg() -> Rgba {
    rgb(0x1b1d25)
}

pub fn button_hover() -> Rgba {
    rgb(0x22242d)
}
