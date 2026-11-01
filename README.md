# Noctua

Noctua es un lector de documentos PDF de escritorio escrito en Rust. Su objetivo es ofrecer una experiencia de lectura cómoda, con modo oscuro, traducción al español, seguimiento del progreso y soporte futuro para OCR.

## Estado del proyecto

Noctua se encuentra en una etapa inicial de desarrollo. Actualmente cuenta con:

- Una arquitectura Tauri con backend Rust y frontend web.
- Una ventana de escritorio creada con Tauri y HTML/CSS/JavaScript.
- Selección persistente de una carpeta como vault de libros.
- Detección automática de PDFs, incluyendo subcarpetas.
- Apertura externa con doble clic, clic derecho o el botón de abrir.

Los PDFs se abren con la aplicación predeterminada del sistema. La visualización integrada se añadirá más adelante.

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
└── pdf/
    └── mod.rs
web/
├── index.html
├── styles.css
├── app.js
└── assets/icons/

```

- `main.rs`: comandos Tauri, configuración del vault y escaneo de PDFs.
- `pdf`: funcionalidad relacionada con documentos PDF.
- `web`: interfaz web y estilos de la biblioteca.

Los módulos de traducción, almacenamiento y OCR se añadirán cuando exista funcionalidad real para ellos.

## Requisitos

- Una instalación estable de [Rust](https://www.rust-lang.org/tools/install).
- Cargo, instalado junto con Rust.
- En Linux, las dependencias de WebKitGTK requeridas por Tauri (`webkit2gtk-4.1` y `javascriptcoregtk-4.1`).

En Arch Linux puedes instalarlas con:

```bash
sudo pacman -Syu --needed webkit2gtk-4.1
```

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

`cargo run` compila y abre la aplicación usando los archivos de `web/`. Para desarrollo con recarga de la interfaz, instala la CLI de Tauri y ejecuta:

```bash
cargo install tauri-cli --version '^2.0.0'
cargo tauri dev
```

La primera compilación puede tardar mientras Cargo descarga y compila las dependencias.

## Comandos de desarrollo

```bash
cargo fmt --check
cargo check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
```

También puedes comprobar la sintaxis del frontend con:

```bash
node --check web/app.js
```

## Contribuir

Las contribuciones son bienvenidas. Antes de comenzar, consulta [CONTRIBUTING.md](CONTRIBUTING.md) para conocer el flujo de trabajo y las reglas del proyecto.

## Licencia

Noctua se distribuye bajo la [GNU General Public License v3.0](LICENSE).
