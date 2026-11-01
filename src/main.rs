#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod pdf;

use std::{
    fs,
    path::{Path, PathBuf},
    time::SystemTime,
};

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct Book {
    name: String,
    path: String,
    relative_path: String,
    size: String,
    modified: String,
}

#[derive(Clone, Debug, Serialize)]
struct LibraryPayload {
    vault: Option<String>,
    books: Vec<Book>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
struct Config {
    #[serde(default)]
    vault: Option<String>,
}

#[tauri::command]
fn load_library() -> Result<LibraryPayload, String> {
    let vault = configured_vault();
    Ok(payload(vault))
}

#[tauri::command]
fn choose_vault() -> Result<LibraryPayload, String> {
    let Some(path) = rfd::FileDialog::new()
        .set_title("Elige tu carpeta de libros")
        .pick_folder()
    else {
        return load_library();
    };

    save_config(&path)?;
    Ok(payload(Some(path)))
}

#[tauri::command]
fn refresh(vault: String) -> Result<LibraryPayload, String> {
    let path = PathBuf::from(vault);
    if !path.is_dir() {
        return Err("La carpeta del vault ya no existe".to_string());
    }
    Ok(payload(Some(path)))
}

#[tauri::command]
fn pick_pdf() -> Result<Option<String>, String> {
    Ok(rfd::FileDialog::new()
        .add_filter("PDF", &["pdf"])
        .pick_file()
        .map(|path| path.to_string_lossy().to_string()))
}

#[tauri::command]
fn open_file(path: String) -> Result<(), String> {
    crate::pdf::open_externally(Path::new(&path)).map_err(|error| error.to_string())
}

#[tauri::command]
fn open_folder(path: String) -> Result<(), String> {
    let path = Path::new(&path);
    open::that_detached(path.parent().unwrap_or(path)).map_err(|error| error.to_string())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            load_library,
            choose_vault,
            refresh,
            pick_pdf,
            open_file,
            open_folder
        ])
        .run(tauri::generate_context!())
        .expect("error while running Noctua");
}

fn payload(vault: Option<PathBuf>) -> LibraryPayload {
    let books = vault
        .as_deref()
        .map(|path| {
            let mut books = Vec::new();
            scan_directory(path, path, &mut books);
            books.sort_by_key(|book| std::cmp::Reverse(book.modified));
            books.into_iter().map(|book| book.book).collect()
        })
        .unwrap_or_default();

    LibraryPayload {
        vault: vault.map(|path| path.to_string_lossy().to_string()),
        books,
    }
}

fn configured_vault() -> Option<PathBuf> {
    let json = fs::read_to_string(config_path()).ok()?;
    let config = serde_json::from_str::<Config>(&json).ok()?;
    let path = PathBuf::from(config.vault?);
    path.is_dir().then_some(path)
}

fn save_config(path: &Path) -> Result<(), String> {
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

fn config_path() -> PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("noctua")
        .join("config.json")
}

fn scan_directory(root: &Path, dir: &Path, books: &mut Vec<BookWithDate>) {
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            scan_directory(root, &path, books);
        } else if crate::pdf::is_pdf(&path)
            && let Ok(metadata) = entry.metadata()
        {
            let modified = metadata.modified().ok();
            books.push(BookWithDate {
                book: Book {
                    name: path
                        .file_name()
                        .and_then(|name| name.to_str())
                        .unwrap_or("Sin nombre")
                        .to_string(),
                    path: path.to_string_lossy().to_string(),
                    relative_path: relative_path(root, &path),
                    size: format_bytes(metadata.len()),
                    modified: format_date(modified),
                },
                modified,
            });
        }
    }
}

struct BookWithDate {
    book: Book,
    modified: Option<SystemTime>,
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
    const UNITS: &[&str] = &["B", "KB", "MB", "GB"];
    let mut size = bytes as f64;
    let mut unit = 0;
    while size >= 1024.0 && unit < UNITS.len() - 1 {
        size /= 1024.0;
        unit += 1;
    }
    format!("{size:.1} {}", UNITS[unit])
}
