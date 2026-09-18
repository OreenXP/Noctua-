# Contribuir a Noctua

Gracias por tu interés en contribuir a Noctua. Estas pautas buscan mantener el proyecto comprensible y facilitar la revisión de cambios.

## Antes de comenzar

- Revisa los _issues_ existentes para comprobar si alguien ya está trabajando en la misma idea o problema.
- Abre un _issue_ antes de realizar un cambio grande o modificar la arquitectura.
- Mantén cada contribución centrada en un solo objetivo.
- No hagas commits directamente en `dev` ni en `main`. Crea una rama nueva desde `dev` para cada contribución.
- Nunca incluyas contraseñas, tokens, claves de API ni documentos privados en el repositorio.

## Preparar el proyecto

1. Haz un _fork_ del repositorio.
2. Clona tu _fork_ y entra en la carpeta del proyecto.
3. Añade el repositorio original como remoto `upstream`.
4. Crea tu rama de trabajo desde `dev`.

```bash
git clone https://github.com/TU-USUARIO/Noctua-.git
cd Noctua-
git remote add upstream https://github.com/OreenXP/Noctua-.git
git fetch upstream
git switch -c feat/nombre-del-cambio upstream/dev
```

Realiza tus commits en esa rama y, cuando el cambio esté listo, abre un _pull request_ desde tu rama hacia `dev`.

Usa nombres de rama breves y descriptivos, por ejemplo:

- `feat/pdf-opening`
- `fix/window-crash`
- `docs/update-readme`
- `refactor/ui-state`

## Reglas para el código

- Sigue las convenciones habituales de Rust.
- Usa `rustfmt` para mantener un formato uniforme.
- Atiende las advertencias de Clippy.
- Conserva la separación entre `app`, `ui` y `pdf`.
- No añadas módulos o dependencias sin una necesidad actual y clara.
- Evita mezclar refactorizaciones no relacionadas con la función o corrección principal.
- Añade o actualiza pruebas cuando el comportamiento pueda verificarse automáticamente.
- Documenta las decisiones que no resulten evidentes al leer el código.

## Verificaciones obligatorias

Antes de enviar una contribución, ejecuta:

```bash
cargo fmt --check
cargo check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
```

Todos los comandos deben terminar correctamente.

## Mensajes de commit

Los mensajes de commit deben escribirse en inglés y seguir un formato similar a Conventional Commits:

```text
tipo: short description
```

Tipos habituales:

- `feat`: funcionalidad nueva.
- `fix`: corrección de un error.
- `docs`: cambios en documentación.
- `refactor`: reorganización sin cambiar el comportamiento.
- `test`: creación o modificación de pruebas.
- `chore`: mantenimiento y herramientas.

Ejemplos:

```text
feat: add PDF file picker
fix: prevent crash when closing the window
docs: explain the development workflow
```

## Enviar un pull request

- Envía el _pull request_ hacia la rama `dev`, no hacia `main`.
- Utiliza un título claro en inglés.
- Explica qué problema resuelve el cambio y cómo lo verificaste.
- Incluye capturas de pantalla si modificaste la interfaz.
- Relaciona el _issue_ correspondiente cuando exista.
- Responde a los comentarios de revisión y mantén el cambio actualizado.

La rama `main` se reserva para versiones estables. Los cambios llegan primero a `dev` y se integran en `main` después de su revisión y verificación.

## Licencia

Al contribuir, aceptas que tu trabajo se distribuya bajo la [GNU General Public License v3.0](LICENSE) del proyecto.
