# Noctua

Noctua es un lector de documentos PDF de escritorio escrito en Rust. Su objetivo es ofrecer una experiencia de lectura cómoda, con modo oscuro, traducción al español, seguimiento del progreso y soporte futuro para OCR.

## Estado del proyecto

Noctua se encuentra en una etapa inicial de desarrollo. Actualmente cuenta con:

- Una arquitectura modular básica.
- Una ventana de escritorio creada con `egui` y `eframe`.
- Módulos iniciales para la aplicación, la interfaz y el futuro procesamiento de PDF.

Todavía no es posible abrir ni visualizar documentos PDF.

## Funciones planeadas

- Abrir y visualizar documentos PDF locales.
- Aplicar un modo oscuro cómodo para la lectura.
- Recordar el documento y la página actual.
- Extraer y seleccionar texto.
- Traducir fragmentos al español.
- Mostrar una vista bilingüe.
- Reconocer texto de documentos escaneados mediante OCR.

## Estructura actual

```text
src/
├── main.rs
├── app.rs
├── pdf/
│   └── mod.rs
└── ui/
    └── mod.rs
```

- `main.rs`: punto de entrada del programa.
- `app.rs`: estado y coordinación de la aplicación.
- `pdf`: funcionalidad relacionada con documentos PDF.
- `ui`: presentación e interacción con el usuario.

Los módulos de traducción, almacenamiento y OCR se añadirán cuando exista funcionalidad real para ellos.

## Requisitos

- Una instalación estable de [Rust](https://www.rust-lang.org/tools/install).
- Cargo, instalado junto con Rust.

## Ejecutar el proyecto

Clona el repositorio y entra en su directorio:

```bash
git clone https://github.com/OreenXP/Noctua-.git
cd Noctua-
```

Cambia a la rama de desarrollo y ejecuta la aplicación:

```bash
git switch dev
cargo run
```

La primera compilación puede tardar mientras Cargo descarga y compila las dependencias.

## Comandos de desarrollo

```bash
cargo fmt --check
cargo check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
```

## Contribuir

Las contribuciones son bienvenidas. Antes de comenzar, consulta [CONTRIBUTING.md](CONTRIBUTING.md) para conocer el flujo de trabajo y las reglas del proyecto.

## Licencia

Noctua se distribuye bajo la [GNU General Public License v3.0](LICENSE).
