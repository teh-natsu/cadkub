# Asset attribution

Every non-code asset in this repository (images, icons, fonts, example drawings, presets) is listed
here with its author, source and licence. `cargo xtask assets` (part of `cargo xtask ci`) fails if
an asset file is missing from this table.

**Policy (mandatory):** CadKub contains **no Autodesk, Adobe or Avid iconography, images,
artwork, fonts, hatch patterns, linetypes, templates or presets.** Every asset is original work by
CADCraft contributors or third-party material under an open licence (OSI open source, public domain
/ CC0, or Creative Commons that allows redistribution). Screenshots of Autodesk software are never
committed. Font files are not added here (Anuphan, the UI font for Thai, is the one exception):
shared fonts live in [storytold/craft-fonts](https://github.com/storytold/craft-fonts), an optional
build input.

Generated-in-code assets are original and have no file to list:
- the UI icon set (`crates/ui-egui/src/icons.rs`);
- the "CadKub Stroke" single-stroke drafting font (`crates/fonts/src/stroke.rs`);
- the standard linetype and hatch-pattern libraries (`crates/doc/src/library.rs`; industry-common
  names, our own dash/spacing values);
- the colour index palette, generated from its structure (`crates/color/src/lib.rs`);
- the sample drawings (`crates/engine/src/sample.rs`).

| Asset | Author | Source | Licence | Notes |
|---|---|---|---|---|
| `assets/app-icon/cadkub.svg` | Nattpol Chaisri (CadKub owner) | original work, drawn as plain SVG shapes by `packaging/make_icon.py` | MIT OR Apache-2.0 | CadKub app icon master (red panda holding a blueprint) |
| `assets/app-icon/cadkub-small.svg` | Nattpol Chaisri (CadKub owner) | original work, drawn as plain SVG shapes by `packaging/make_icon.py` | MIT OR Apache-2.0 | simpler variant for 24 px and below |
| `assets/app-icon/hicolor/scalable/apps/io.github.teh_natsu.cadkub.svg` | Nattpol Chaisri (CadKub owner) | copy of `assets/app-icon/cadkub.svg` | MIT OR Apache-2.0 | Linux hicolor icon |
| `assets/app-icon/hicolor/16x16/apps/io.github.teh_natsu.cadkub.png` | Nattpol Chaisri (CadKub owner) | rendered from `assets/app-icon/cadkub.svg` (`cadkub-small.svg` at 24 px and below) by `packaging/icons.sh` | MIT OR Apache-2.0 | Linux hicolor icon |
| `assets/app-icon/hicolor/24x24/apps/io.github.teh_natsu.cadkub.png` | Nattpol Chaisri (CadKub owner) | rendered from `assets/app-icon/cadkub.svg` (`cadkub-small.svg` at 24 px and below) by `packaging/icons.sh` | MIT OR Apache-2.0 | Linux hicolor icon |
| `assets/app-icon/hicolor/32x32/apps/io.github.teh_natsu.cadkub.png` | Nattpol Chaisri (CadKub owner) | rendered from `assets/app-icon/cadkub.svg` (`cadkub-small.svg` at 24 px and below) by `packaging/icons.sh` | MIT OR Apache-2.0 | Linux hicolor icon |
| `assets/app-icon/hicolor/48x48/apps/io.github.teh_natsu.cadkub.png` | Nattpol Chaisri (CadKub owner) | rendered from `assets/app-icon/cadkub.svg` (`cadkub-small.svg` at 24 px and below) by `packaging/icons.sh` | MIT OR Apache-2.0 | Linux hicolor icon |
| `assets/app-icon/hicolor/64x64/apps/io.github.teh_natsu.cadkub.png` | Nattpol Chaisri (CadKub owner) | rendered from `assets/app-icon/cadkub.svg` (`cadkub-small.svg` at 24 px and below) by `packaging/icons.sh` | MIT OR Apache-2.0 | Linux hicolor icon |
| `assets/app-icon/hicolor/128x128/apps/io.github.teh_natsu.cadkub.png` | Nattpol Chaisri (CadKub owner) | rendered from `assets/app-icon/cadkub.svg` (`cadkub-small.svg` at 24 px and below) by `packaging/icons.sh` | MIT OR Apache-2.0 | Linux hicolor icon |
| `assets/app-icon/hicolor/256x256/apps/io.github.teh_natsu.cadkub.png` | Nattpol Chaisri (CadKub owner) | rendered from `assets/app-icon/cadkub.svg` (`cadkub-small.svg` at 24 px and below) by `packaging/icons.sh` | MIT OR Apache-2.0 | Linux hicolor icon |
| `assets/app-icon/hicolor/512x512/apps/io.github.teh_natsu.cadkub.png` | Nattpol Chaisri (CadKub owner) | rendered from `assets/app-icon/cadkub.svg` (`cadkub-small.svg` at 24 px and below) by `packaging/icons.sh` | MIT OR Apache-2.0 | Linux hicolor icon |
| `assets/app-icon/cadkub.icns` | Nattpol Chaisri (CadKub owner) | rendered from `assets/app-icon/cadkub.svg` by `packaging/icons.sh` | MIT OR Apache-2.0 | macOS icon |
| `assets/app-icon/cadkub.ico` | Nattpol Chaisri (CadKub owner) | rendered from `assets/app-icon/cadkub.svg` by `packaging/icons.sh` | MIT OR Apache-2.0 | Windows icon |
| `assets/app-icon/cadkub-64.png` | Nattpol Chaisri (CadKub owner) | rendered from `assets/app-icon/cadkub.svg` by `packaging/icons.sh` | MIT OR Apache-2.0 |  |
| `assets/app-icon/cadkub-256.png` | Nattpol Chaisri (CadKub owner) | rendered from `assets/app-icon/cadkub.svg` by `packaging/icons.sh` | MIT OR Apache-2.0 | runtime window icon |
| `assets/app-icon/cadkub-1024.png` | Nattpol Chaisri (CadKub owner) | rendered from `assets/app-icon/cadkub.svg` by `packaging/icons.sh` | MIT OR Apache-2.0 |  |
| `assets/app-icon/cadkub-macos-512.png` | Nattpol Chaisri (CadKub owner) | rendered from `assets/app-icon/cadkub.svg` by `packaging/icons.sh` | MIT OR Apache-2.0 | macOS margin variant |
| `assets/fonts/Anuphan-Regular.ttf` | The Anuphan Project Authors (Cadson Demak) | https://github.com/google/fonts/tree/main/ofl/anuphan (`Anuphan[wght].ttf`, version 3.002) | SIL Open Font License 1.1 (`assets/fonts/OFL-Anuphan.txt`) | static Regular instance of the variable font, made with fontTools; the UI font for Thai |
| `assets/fonts/OFL-Anuphan.txt` | The Anuphan Project Authors | https://github.com/google/fonts/tree/main/ofl/anuphan | SIL Open Font License 1.1 | licence text |
| `examples/apartment.dxf` | CADCraft contributors | sample drawing made with cadkub-cli commands | MIT OR Apache-2.0 | Original |
| `docs/images/ui-apartment.png` | CADCraft contributors | screenshot of CadKub itself (examples/apartment.dxf) | MIT OR Apache-2.0 | Original; no Autodesk UI |
| `docs/images/ui-layout.png` | CADCraft contributors | screenshot of CadKub itself (a layout of examples/apartment.dxf) | MIT OR Apache-2.0 | Original; no Autodesk UI |
| `docs/images/ui-bracket.png` | CADCraft contributors | screenshot of CadKub itself (sample drawing generated in code) | MIT OR Apache-2.0 | Original; no Autodesk UI |
