# Noctua

Noctua es un lector de documentos PDF de escritorio escrito en Rust. El objetivo es ofrecer lectura en modo oscuro, traducción al español, progreso de lectura y soporte futuro para OCR.

## Estado

El proyecto está en su etapa inicial. Por ahora contiene solamente el esqueleto modular del programa; todavía no integra interfaz gráfica ni procesamiento de PDF.

## Estructura inicial

```text
src/
├── main.rs
├── app.rs
├── pdf/
│   └── mod.rs
└── ui/
    └── mod.rs
```

- `main.rs`: punto de entrada del ejecutable.
- `app.rs`: estado y coordinación de la aplicación.
- `pdf`: funcionalidad relacionada con documentos PDF.
- `ui`: presentación e interacción con el usuario.

Los módulos de traducción, almacenamiento y OCR se añadirán cuando exista funcionalidad real para ellos.

## Comandos de desarrollo

```bash
cargo run
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
```

## Licencia

Noctua se distribuye bajo GNU General Public License v3.0. Consulta [LICENSE](LICENSE).
