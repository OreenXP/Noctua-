//! Lógica de la biblioteca: vault, configuración, escaneo y formato.
//!
//! Este módulo no depende de la interfaz. Recibe rutas y devuelve datos listos
//! para mostrarse.

use std::{
    fs,
    path::{Path, PathBuf},
    time::SystemTime,
};

use serde::{Deserialize, Serialize};

use crate::pdf;

#[derive(Clone, Debug)]
pub struct Book {
    pub name: String,
    pub path: String,
    pub relative_path: String,
    pub size: String,
    pub size_bytes: u64,
    pub modified: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
struct Config {
    #[serde(default)]
    vault: Option<String>,
}

/// Carpeta de configuración de Noctua (`~/.config/noctua` en Linux).
pub fn config_path() -> PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("noctua")
        .join("config.json")
}

/// Vault guardado en la configuración, solo si todavía existe.
pub fn configured_vault() -> Option<PathBuf> {
    let json = fs::read_to_string(config_path()).ok()?;
    let config = serde_json::from_str::<Config>(&json).ok()?;
    let path = PathBuf::from(config.vault?);
    path.is_dir().then_some(path)
}

/// Guarda el vault elegido, creando el directorio de configuración si hace falta.
pub fn save_vault(path: &Path) -> Result<(), String> {
    let config_path = config_path();
    if let Some(parent) = config_path.parent() {
        fs::create_dir_all(parent).map_err(|error| error.to_string())?;
    }
    let json = serde_json::to_string_pretty(&Config {
        vault: Some(path.to_string_lossy().to_string()),
    })
    .map_err(|error| error.to_string())?;
    fs::write(config_path, json).map_err(|error| error.to_string())
}

/// Carga los libros del vault, o devuelve un error legible si ya no existe.
pub fn load_library(vault: &Path) -> Result<Vec<Book>, String> {
    if !vault.is_dir() {
        return Err("La carpeta del vault ya no existe".to_string());
    }
    Ok(scan_books(vault))
}

/// Escanea el vault recursivamente y ordena por fecha de modificación descendente.
pub fn scan_books(root: &Path) -> Vec<Book> {
    let mut books = Vec::new();
    walk(root, root, &mut books);
    books.sort_by_cached_key(|book| std::cmp::Reverse(book.modified.clone()));
    books
}

fn walk(root: &Path, dir: &Path, books: &mut Vec<Book>) {
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };

    for entry in entries.flatten() {
        let path = entry.path();

        if path.is_dir() {
            walk(root, &path, books);
        } else if pdf::is_pdf(&path)
            && let Ok(metadata) = entry.metadata()
        {
            let modified_time = metadata.modified().ok();
            books.push(Book {
                name: path
                    .file_name()
                    .and_then(|name| name.to_str())
                    .unwrap_or("Sin nombre")
                    .to_string(),
                path: path.to_string_lossy().to_string(),
                relative_path: relative_path(root, &path),
                size: format_bytes(metadata.len()),
                size_bytes: metadata.len(),
                modified: format_date(modified_time),
            });
        }
    }
}

fn relative_path(root: &Path, path: &Path) -> String {
    path.strip_prefix(root)
        .ok()
        .and_then(|relative| relative.parent())
        .filter(|parent| !parent.as_os_str().is_empty())
        .map(|parent| parent.display().to_string())
        .unwrap_or_else(|| "Mi biblioteca".to_string())
}

fn format_date(value: Option<SystemTime>) -> String {
    value
        .map(|value| {
            chrono::DateTime::<chrono::Local>::from(value)
                .format("%d %b %Y")
                .to_string()
        })
        .unwrap_or_else(|| "—".to_string())
}

fn format_bytes(bytes: u64) -> String {
    const UNITS: [&str; 4] = ["B", "KB", "MB", "GB"];
    let mut size = bytes as f64;
    let mut unit = 0;
    while size >= 1024.0 && unit < UNITS.len() - 1 {
        size /= 1024.0;
        unit += 1;
    }
    format!("{size:.1} {}", UNITS[unit])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn formats_bytes_with_one_decimal() {
        assert_eq!(format_bytes(512), "512.0 B");
        assert_eq!(format_bytes(1536), "1.5 KB");
        assert_eq!(format_bytes(2 * 1024 * 1024), "2.0 MB");
    }

    #[test]
    fn relative_path_uses_parent_folder() {
        let root = Path::new("/libros");
        assert_eq!(
            relative_path(root, Path::new("/libros/Ficción/novela.pdf")),
            "Ficción"
        );
        assert_eq!(
            relative_path(root, Path::new("/libros/novela.pdf")),
            "Mi biblioteca"
        );
    }
}
