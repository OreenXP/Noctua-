//! Assets embebidos en el binario: los iconos SVG de la interfaz.

use std::borrow::Cow;

use gpui::{AssetSource, SharedString};

const ICONS: &[(&str, &[u8])] = &[
    (
        "icons/chevron-down.svg",
        include_bytes!("../../assets/icons/chevron-down.svg"),
    ),
    (
        "icons/external-link.svg",
        include_bytes!("../../assets/icons/external-link.svg"),
    ),
    (
        "icons/file-text.svg",
        include_bytes!("../../assets/icons/file-text.svg"),
    ),
    (
        "icons/folder.svg",
        include_bytes!("../../assets/icons/folder.svg"),
    ),
    (
        "icons/grid.svg",
        include_bytes!("../../assets/icons/grid.svg"),
    ),
    (
        "icons/list.svg",
        include_bytes!("../../assets/icons/list.svg"),
    ),
    (
        "icons/moon.svg",
        include_bytes!("../../assets/icons/moon.svg"),
    ),
    (
        "icons/refresh.svg",
        include_bytes!("../../assets/icons/refresh.svg"),
    ),
    (
        "icons/search.svg",
        include_bytes!("../../assets/icons/search.svg"),
    ),
    (
        "icons/settings.svg",
        include_bytes!("../../assets/icons/settings.svg"),
    ),
    (
        "icons/sort.svg",
        include_bytes!("../../assets/icons/sort.svg"),
    ),
];

pub struct Assets;

impl AssetSource for Assets {
    fn load(&self, path: &str) -> gpui::Result<Option<Cow<'static, [u8]>>> {
        Ok(ICONS
            .iter()
            .find(|(name, _)| *name == path)
            .map(|(_, data)| Cow::Borrowed(*data)))
    }

    fn list(&self, path: &str) -> gpui::Result<Vec<SharedString>> {
        Ok(ICONS
            .iter()
            .filter(|(name, _)| name.starts_with(path))
            .map(|(name, _)| SharedString::from(*name))
            .collect())
    }
}
