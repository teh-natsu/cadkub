#!/usr/bin/env bash
# Regenerate every app icon file from assets/app-icon/cadkub.svg (the master vector) and
# cadkub-small.svg (the variant for 24 px and below). Both SVGs come from packaging/make_icon.py.
#
# Needs: resvg (brew install resvg / cargo install resvg). The .icns is written by iconutil on
# macOS, or by Pillow elsewhere ($PYTHON, default python3). The outputs are committed, so building
# and packaging never need these tools.
#
#   packaging/icons.sh
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
DIR="$ROOT/assets/app-icon"
SVG="$DIR/cadkub.svg"
SMALL="$DIR/cadkub-small.svg"
ID="io.github.teh_natsu.cadkub"
PYTHON="${PYTHON:-python3}"
TMP="$(mktemp -d)"
trap 'rm -rf "$TMP"' EXIT

command -v resvg >/dev/null || { echo "error: resvg not found (brew install resvg)" >&2; exit 1; }

# The masters are full-bleed 512 tiles (rx=112). Windows and Linux use them as they are. macOS
# icons follow Apple's grid: an 824/1024 body with a transparent margin, made by widening the
# viewBox (512 / 0.805 = 636, so 62 units each side).
MAC="$TMP/macos.svg"
MAC_SMALL="$TMP/macos-small.svg"
for pair in "$SVG:$MAC" "$SMALL:$MAC_SMALL"; do
  sed 's/viewBox="0 0 512 512"/viewBox="-62 -62 636 636"/' "${pair%%:*}" >"${pair#*:}"
  grep -q 'viewBox="-62 -62 636 636"' "${pair#*:}" || { echo "error: unexpected viewBox in ${pair%%:*}" >&2; exit 1; }
done

render() { resvg -w "$2" -h "$2" "$1" "$3" </dev/null; }
# The fine detail is lost at 24 px and below: use the small variant there.
pick() { if [ "$1" -le 24 ]; then echo "$SMALL"; else echo "$SVG"; fi; }
pick_mac() { if [ "$1" -le 32 ]; then echo "$MAC_SMALL"; else echo "$MAC"; fi; }

render "$SVG" 1024 "$DIR/cadkub-1024.png"
render "$SVG" 256 "$DIR/cadkub-256.png"
render "$SVG" 64 "$DIR/cadkub-64.png"
# macOS-margin master (the DMG and docs).
render "$MAC" 512 "$DIR/cadkub-macos-512.png"

# Linux hicolor theme. The runtime window icon is cadkub-256.png (apps/cadkub/src/main.rs).
for s in 16 24 32 48 64 128 256 512; do
  mkdir -p "$DIR/hicolor/${s}x${s}/apps"
  render "$(pick "$s")" "$s" "$DIR/hicolor/${s}x${s}/apps/$ID.png"
done
mkdir -p "$DIR/hicolor/scalable/apps"
cp "$SVG" "$DIR/hicolor/scalable/apps/$ID.svg"

# Windows .ico.
ICO_PNGS=()
for s in 16 20 24 32 40 48 64 128 256; do
  render "$(pick "$s")" "$s" "$TMP/ico-$s.png"
  ICO_PNGS+=("$TMP/ico-$s.png")
done
(cd "$ROOT" && cargo run -q -p xtask -- ico "$DIR/cadkub.ico" "${ICO_PNGS[@]}")

# macOS .icns.
if command -v iconutil >/dev/null; then
  SET="$TMP/cadkub.iconset"
  mkdir -p "$SET"
  for s in 16 32 128 256 512; do
    render "$(pick_mac "$s")" "$s" "$SET/icon_${s}x${s}.png"
    render "$(pick_mac $((s * 2)))" $((s * 2)) "$SET/icon_${s}x${s}@2x.png"
  done
  iconutil -c icns -o "$DIR/cadkub.icns" "$SET"
else
  for s in 16 32 64 128 256 512 1024; do
    render "$(pick_mac "$s")" "$s" "$TMP/mac-$s.png"
  done
  "$PYTHON" - "$DIR/cadkub.icns" "$TMP" <<'PY'
import sys
from PIL import Image
out, tmp = sys.argv[1], sys.argv[2]
others = [Image.open(f"{tmp}/mac-{s}.png") for s in (16, 32, 64, 128, 256, 512)]
Image.open(f"{tmp}/mac-1024.png").save(out, append_images=others)
PY
fi
echo "icons written to $DIR"
