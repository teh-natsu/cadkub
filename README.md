<p align="center">
  <img src="assets/app-icon/cadkub.svg" alt="ไอคอน CadKub: แพนด้าแดงโผล่หน้ามาจับแบบพิมพ์เขียว" width="128">
</p>

<h1 align="center">CadKub</h1>

<p align="center">
  <b>โปรแกรมเขียนแบบด้วยคอมพิวเตอร์ (CAD) แบบโอเพนซอร์ส เขียนด้วย Rust ทั้งหมด</b><br>
  พิมพ์คำสั่ง สแนปจุด จัดเลเยอร์ ใส่มิติ แรเงา บล็อก เลย์เอาต์ และพล็อตเป็น PDF เปิดและบันทึก DXF / DWG ได้<br>
  macOS · Windows · Linux · FreeBSD · เว็บ
</p>

<p align="center">
  <img alt="License: MIT OR Apache-2.0" src="https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-0f8a55">
  <img alt="Written in Rust" src="https://img.shields.io/badge/written%20in-Rust-3fcf86">
  <img alt="Agent-drivable over MCP" src="https://img.shields.io/badge/agents-MCP-2f7fd6">
  <img alt="No account, no telemetry" src="https://img.shields.io/badge/no%20account-no%20telemetry-e8692d">
</p>

> [!NOTE]
> CadKub พัฒนาต่อจาก [CADCraft](https://github.com/storytold/cadcraft) ของทีม ArtCraft
> ภายใต้สัญญาอนุญาต MIT OR Apache-2.0 แต่ไม่ได้จัดทำ สนับสนุน หรือรับรองโดยทีม ArtCraft

<p align="center">
  <img src="docs/images/ui-apartment.png" alt="แปลนห้องชุดใน CadKub: ผนังแรเงาสีเทา หน้าต่างสีฟ้า วงสวิงประตูสีเขียว ชื่อห้องพร้อมพื้นที่ ตารางห้อง ลูกศรโน้ต และเส้นมิติแบบสถาปัตย์ แถบเครื่องมือด้านซ้าย เลเยอร์และคุณสมบัติด้านขวา บรรทัดคำสั่งด้านล่าง" width="100%">
  <sub>แปลนห้องชุด (<a href="examples/apartment.dxf">examples/apartment.dxf</a>) วาดด้วยคำสั่งของโปรแกรมทั้งหมด</sub>
</p>

---

## จุดเด่น

- **ทำงานแบบที่คนเขียนแบบคุ้นมือ:** พิมพ์ `L` คลิกสองจุด พิมพ์ `@5<45` แล้วกด Enter บรรทัดคำสั่งมีตัวเลือก `[Keywords]` ให้คลิก
  AutoComplete, object snap, polar tracking, ortho, direct distance entry, การเลือกแบบ window/crossing, grip และคลิกขวาเพื่อทำซ้ำ
- **ไฟล์เปิดกว้าง:** อ่าน/เขียน DXF เอง (ASCII และ binary, R12 ถึง 2018) เปิด/บันทึก DWG (R13 ถึง 2018) ผ่านไลบรารีโอเพนซอร์ส acadrust
  ส่งออก SVG, PNG และพล็อตเป็น PDF
- **เร็วและเป็นโปรแกรมจริง:** Rust กับ egui ไม่มี Electron ไม่มี web view
- **ให้ AI agent ควบคุมได้:** ทุกเมนู เครื่องมือ และพรอมต์เป็นคำสั่ง ผ่าน MCP ช่องควบคุม JSON หรือ CLI
- **เป็นของคุณ:** ฟรี ไม่ต้องสมัครบัญชี ไม่มีค่าสมาชิก
- **ภาษาไทยทั้งหน้าจอและในแบบ:** หน้าจอแสดงชื่อเลเยอร์ บล็อก และชื่อไฟล์ภาษาไทยด้วยฟอนต์ Anuphan
  ส่วนข้อความ TEXT, MTEXT, มิติ และตารางในแบบ ถ้าฟอนต์ของ text style ไม่มีอักษรไทย (เช่นฟอนต์เส้นเดี่ยวหรือ Arial)
  จะใช้ฟอนต์ Sarabun แทนเฉพาะส่วนที่เป็นภาษาไทย และจัดสระบน/ล่างกับวรรณยุกต์ให้ซ้อนถูกตำแหน่ง ส่งออก PNG, SVG และพล็อต PDF ได้ตามที่เห็น

ไฟล์ DXF ที่ CadKub บันทึกยังใช้ชื่อแอป `CADCRAFT` ในข้อมูลเสริม (xdata, constraint และตัวแปร header) เหมือนต้นฉบับ
จึงเปิดสลับกับ CADCraft ได้โดยไม่เสียข้อมูลมิติแบบ associative และ constraint

## ทำอะไรได้บ้าง

| งาน | รายละเอียด |
|---|---|
| พื้นที่วาด | Model space พร้อมกริดปรับตามซูม แกน แพน/ซูม (ล้อเมาส์ ลากปุ่มกลาง pinch) crosshair, UCS icon, ViewCube |
| บรรทัดคำสั่ง | พรอมต์พร้อม keyword ประวัติ AutoComplete alias, `@dx,dy`, `@d<a`, `#x,y`, direct distance entry, คำสั่งแทรก (transparent), สคริปต์แบบ `.scr` |
| วาด | LINE, PLINE, CIRCLE, ARC, RECTANG, POLYGON, ELLIPSE, SPLINE, POINT, XLINE, RAY, DONUT, TEXT, MTEXT |
| แก้ไข | ERASE, MOVE, COPY, ROTATE, SCALE, MIRROR, STRETCH, OFFSET, TRIM, EXTEND, FILLET, CHAMFER, BREAK, JOIN, EXPLODE, ARRAY, draw order, OVERKILL |
| ความแม่นยำ | Object snap ทุกแบบหลัก, polar tracking, ortho, grid snap |
| เลเยอร์และคุณสมบัติ | Layer Properties Manager, เครื่องมือเลเยอร์ (isolate, freeze, lock, match, previous), Properties palette, linetype, lineweight, สี index และ true colour |
| คำอธิบายแบบ | คำสั่ง DIM* ทั้งหมดพร้อม DIMSTYLE, มิติ associative, MLEADER, TABLE, ฟอนต์ TrueType, รหัสจัดรูปแบบ MTEXT, ฟอนต์เส้นเดี่ยวสำหรับเขียนแบบที่ทำเอง |
| แรเงาและบล็อก | Hatch แบบเลือกจุดภายใน (มี island), ลาย สีทึบ และไล่สี, BLOCK/INSERT, attribute และบล็อกซ้อน |
| ไฟล์ | DXF (R12–2018), DWG (R13–2018), พล็อต PDF, ส่งออก SVG และ PNG |
| เลย์เอาต์และพล็อต | Paper space, viewport (สเกล ล็อก freeze เลเยอร์ราย viewport), MSPACE/PSPACE, page setup, PLOT และ EXPORTPDF |
| พาราเมตริก | Constraint เชิงเรขาคณิตและเชิงมิติ, AUTOCONSTRAIN, PARAMETERS พร้อมสูตร |
| อัตโนมัติ | MCP server, ช่องควบคุม JSON, `cadkub-cli` (info, convert, run, commands, mcp) |

<table>
  <tr>
    <td width="50%"><img src="docs/images/ui-layout.png" alt="เลย์เอาต์ paper space: แผ่นกระดาษสีขาว ขอบพิมพ์เส้นประ และ viewport ที่แสดงแปลนห้องชุดตามสเกล"><br><sub>เลย์เอาต์กับ viewport และพล็อตเป็น PDF</sub></td>
    <td width="50%"><img src="docs/images/ui-bracket.png" alt="แบบชิ้นงานแท่นยึด: มุมมองด้านหน้าพร้อมรูสลัก เส้นศูนย์กลาง และมิติ รูปตัดแรเงา โน้ต และกรอบชื่อแบบ"><br><sub>แบบชิ้นงานสองมุมมอง พร้อมรูปตัดแรเงา ANSI31</sub></td>
  </tr>
</table>

## เริ่มต้นใช้งาน

ต้องมี Rust รุ่นล่าสุด (บน Windows ต้องมี Visual Studio Build Tools ที่มี C++ ด้วย)

Desktop and web UI builds require Rust 1.95 or later. The core workspace's declared minimum
remains Rust 1.90.

```sh
git clone https://github.com/teh-natsu/cadkub
cd cadkub
cargo run --release -p cadkub -- --sample     # เปิดแบบตัวอย่าง
cargo test --workspace                         # รันเทสต์
cargo xtask ci                                 # ตรวจทุกอย่างแบบเดียวกับ CI
```

ลองพิมพ์ที่บรรทัดคำสั่ง:

```text
line 0,0 @10,0 @0,5 c          สามเหลี่ยมปิด
circle 5,2 1                    วงกลม
offset 0.25                     แล้วคลิกวงกลมและด้านที่จะ offset
zoom e                          ซูมให้เห็นทั้งแบบ
```

เวอร์ชันเว็บ: `cd apps/cadkub-web && trunk serve` (ต้องมี [trunk](https://trunkrs.dev))

## ให้ AI agent ควบคุม

```sh
cadkub-cli mcp                                  # MCP server ทาง stdio แบบไม่เปิดหน้าจอ
cadkub-cli mcp --connect 127.0.0.1:7979         # คุมโปรแกรมที่เปิดอยู่ (เปิดด้วย --control 7979)
cadkub-cli run --sample --script 'CIRCLE 22,3 1\n' --save out.dxf --export out.png
cadkub-cli info drawing.dxf
cadkub-cli convert drawing.dxf drawing.svg
```

เครื่องมือ MCP มี `command_line` (พิมพ์ที่พรอมต์) `execute` (เรียกคำสั่งใดก็ได้ด้วย JSON) `inspect_drawing` `query_entities`
`render` (ได้ PNG กลับมา) และเมื่อต่อกับโปรแกรมจะมี `screenshot` กับ `ui_click` ด้วย
รายละเอียดอยู่ใน [docs/mcp.md](docs/mcp.md) และ [docs/control-protocol.md](docs/control-protocol.md)

## โครงสร้างโค้ด

เป็น Cargo workspace ที่แบ่ง crate ตามหน้าที่และบังคับลำดับชั้น (`cargo xtask layers`) โดยแกนหลักไม่ขึ้นกับ UI
(ชื่อ crate ภายในยังเป็น `cadcraft-*` เหมือนต้นฉบับ เพื่อให้ดึงอัปเดตจาก CADCraft ได้ง่าย):

| Crate | หน้าที่ |
|---|---|
| `cadcraft-geom`, `-dxf`, `-dwg`, `-color` | เรขาคณิต f64, อ่าน/เขียน tag DXF, DWG ผ่าน acadrust, สี |
| `cadcraft-doc`, `-constraints` | ฐานข้อมูลแบบ (copy-on-write, undo ราคาถูก) และตัวแก้ constraint |
| `cadcraft-fonts`, `-render` | ฟอนต์เส้นเดี่ยวและการจัดวาง TEXT/MTEXT, display list, linetype, hatch, มิติ |
| `cadcraft-io` | แปลง DXF/DWG เป็นแบบ, ส่งออก SVG/PNG/PDF |
| `cadcraft-engine` | session คำสั่ง บรรทัดคำสั่งและพรอมต์ snap การเลือก undo |
| `cadcraft-ui-egui`, `-mcp` | หน้าจอ desktop/เว็บ และ MCP server |
| `apps/cadkub`, `apps/cadkub-cli`, `apps/cadkub-web` | ตัวโปรแกรม CLI และเวอร์ชันเว็บ |

## สถานะ

ยังอยู่ในช่วงพัฒนาเร็ว รายละเอียดความคืบหน้าและความครอบคลุมเทียบกับ AutoCAD อยู่ใน [ROADMAP.md](ROADMAP.md) และ [docs/parity.md](docs/parity.md)
ไฟล์ติดตั้งจะอยู่ที่ [Releases](https://github.com/teh-natsu/cadkub/releases)

## สัญญาอนุญาตและเครดิต

CadKub ใช้สัญญาอนุญาตคู่ [MIT](LICENSE-MIT) หรือ [Apache-2.0](LICENSE-APACHE) เลือกได้ตามต้องการ
Copyright (c) 2026 Nattpol Chaisri and the CadKub contributors

พัฒนาต่อจาก [CADCraft](https://github.com/storytold/cadcraft),
Copyright (c) 2026 ArtCraft Team and the CADCraft contributors ข้อความที่ต้องแสดงอยู่ใน [NOTICE](NOTICE)

ไอคอนใน UI ฟอนต์เขียนแบบเส้นเดี่ยว ลายแรเงา และ linetype ทั้งหมดเป็นงานต้นฉบับที่วาดหรือนิยามในโค้ด
ฟอนต์ Anuphan, Sarabun และ asset อื่น ๆ ใช้สัญญาอนุญาตแบบเปิดของแต่ละชิ้น รายการพร้อมผู้สร้างและแหล่งที่มาอยู่ใน [ATTRIBUTION.md](ATTRIBUTION.md)
ไอคอนโปรแกรมสร้างด้วย [packaging/make_icon.py](packaging/make_icon.py) และ [packaging/icons.sh](packaging/icons.sh)

<sub>Autodesk, AutoCAD and DWG are trademarks or registered trademarks of Autodesk, Inc. in the United States and/or other countries. CadKub is an independent, open-source project and is not affiliated with, sponsored by or endorsed by Autodesk, Inc.; these names are used only to describe the workflows and file formats it is compatible with.</sub>
