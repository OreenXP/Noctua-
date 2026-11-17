#!/usr/bin/env bash
# Descarga la biblioteca pdfium fijada por el proyecto.
#
# El binario no se versiona: cada persona lo obtiene con este script.
set -euo pipefail

# Versión de pdfium fijada para que todos usen el mismo motor.
VERSION="chromium/8165"
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
DEST="$ROOT/vendor/pdfium"

if [[ -f "$DEST/libpdfium.so" ]]; then
    echo "pdfium ya está en $DEST"
    exit 0
fi

mkdir -p "$DEST"
tmp="$(mktemp -d)"
trap 'rm -rf "$tmp"' EXIT

url="https://github.com/bblanchon/pdfium-binaries/releases/download/${VERSION//\//%2F}/pdfium-linux-x64.tgz"

echo "Descargando pdfium $VERSION..."
curl -L --fail --progress-bar -o "$tmp/pdfium.tgz" "$url"
tar -xzf "$tmp/pdfium.tgz" -C "$tmp"
cp "$tmp/lib/libpdfium.so" "$DEST/"
echo "Listo: $DEST/libpdfium.so"
