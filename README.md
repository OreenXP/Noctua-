# Noctua

Noctua es un lector de documentos PDF de escritorio escrito en Rust. Su objetivo es ofrecer una experiencia de lectura cómoda, con modo oscuro, traducción al español, seguimiento del progreso y soporte futuro para OCR.

## Estado del proyecto

Noctua se encuentra en una etapa inicial de desarrollo. Actualmente cuenta con:

- Una interfaz de escritorio nativa construida con [GPUI](https://github.com/zed-industries/zed/tree/main/crates/gpui).
- Selección persistente de una carpeta como vault de libros.
- Detección automática de PDFs, incluyendo subcarpetas.
- Búsqueda, ordenación por fecha, nombre o tamaño y vistas de lista o cuadrícula.
- Apertura externa con doble clic, menú contextual o el botón de abrir.

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
assets/
└── icons/
src/
├── main.rs
├── app.rs
├── library.rs
├── ui/
│   ├── assets.rs
│   ├── components.rs
│   ├── mod.rs
│   ├── text_input.rs
│   ├── theme.rs
│   └── welcome.rs
└── pdf/
    └── mod.rs
```

- `main.rs`: arranque de la aplicación, ventana y atajos de teclado.
- `app.rs`: estado de la biblioteca y composición de la interfaz.
- `library.rs`: configuración del vault, escaneo recursivo y formato.
- `ui`: tema, componentes, campo de búsqueda y pantalla de bienvenida.
- `pdf`: funcionalidad relacionada con documentos PDF.

Los módulos de traducción, almacenamiento y OCR se añadirán cuando exista funcionalidad real para ellos.

## Requisitos

- Una instalación estable de [Rust](https://www.rust-lang.org/tools/install) con soporte para la edición 2024.
- Git.
- Linux con Wayland o X11. GPUI también es compatible con macOS y Windows, aunque esta versión se prueba principalmente en Linux.

### Arch Linux

```bash
sudo pacman -Syu --needed base-devel clang pkg-config fontconfig freetype2 \
  libxkbcommon libxkbcommon-x11 libxcb wayland vulkan-icd-loader
```

### Debian y Ubuntu

```bash
sudo apt update
sudo apt install build-essential clang pkg-config libfontconfig-dev libfreetype-dev \
  libxkbcommon-dev libxkbcommon-x11-dev libxcb1-dev libwayland-dev libvulkan-dev
```

### Windows

1. Instala [Rust](https://www.rust-lang.org/tools/install) con el toolchain MSVC.
2. Instala [Visual Studio Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/) con la carga «Desarrollo para el escritorio con C++».
3. Asegúrate de tener instalados los controladores gráficos actualizados.

## Ejecutar el proyecto

Clona el repositorio y entra en su directorio:

```bash
git clone https://github.com/OreenXP/Noctua-.git
cd Noctua-
```

```bash
cargo run
```

La primera compilación puede tardar mientras Cargo descarga y compila GPUI y sus dependencias. La interfaz usa una revisión fija de GPUI (definida en `Cargo.toml`), por lo que no se necesita Node ni un navegador embebido.

Actualmente Noctua no publica instaladores precompilados para Windows o Linux; la forma recomendada de probar esta versión es compilarla desde el código fuente con `cargo run`. Los instaladores se añadirán cuando el proceso de distribución esté configurado.

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
