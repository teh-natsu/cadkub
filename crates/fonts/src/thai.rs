//! Thai text in drawings (CadKub). The stroke font has no Thai letters and most drafting
//! TrueType fonts have none either, so a Thai run is set in the style's own font when it covers
//! the run and in the bundled Sarabun otherwise. Every Thai run is shaped with harfrust, so
//! upper and lower vowels and tone marks sit on their consonant instead of after it.
//!
//! Thai runs bypass the per-character fallback of [`crate::ttf`]: an installed font found for a
//! single letter would draw Thai unshaped and could change from machine to machine.

use std::collections::HashMap;
use std::sync::{Arc, Mutex, OnceLock, PoisonError};

use cadcraft_geom::Vec2;
use skrifa::instance::{LocationRef, Size};
use skrifa::{GlyphId, MetadataProvider};

use crate::ttf::{Face, UPEM, cap_height, glyph_by_id};

/// The bundled Thai face (Sarabun Regular, SIL Open Font License 1.1).
pub static THAI_FONT: &[u8] = include_bytes!("../../../assets/fonts/Sarabun-Regular.ttf");

/// [`THAI_FONT`] as a face. One copy for the whole process: the glyph caches key faces by address.
pub fn thai_face() -> &'static Face {
    static F: OnceLock<Face> = OnceLock::new();
    F.get_or_init(|| Face { bytes: Arc::new(THAI_FONT.to_vec()), index: 0 })
}

/// Thai block letters, vowels, tone marks and digits.
pub fn is_thai(c: char) -> bool {
    ('\u{0E00}'..='\u{0E7F}').contains(&c)
}

/// Decoded characters split into alternating non-Thai and Thai runs: `(is_thai, run)`.
pub(crate) fn segments(chars: &[(char, bool, bool)]) -> impl Iterator<Item = (bool, &[(char, bool, bool)])> {
    chars.chunk_by(|a, b| is_thai(a.0) == is_thai(b.0)).map(|run| (run.first().is_some_and(|c| is_thai(c.0)), run))
}

/// The face for a Thai run: `primary` when it maps every letter of the run, else the bundled one.
pub(crate) fn face(primary: &Face, run: &[(char, bool, bool)]) -> Face {
    let covered = primary.font().is_some_and(|f| {
        let cmap = f.charmap();
        run.iter().all(|(c, _, _)| c.is_whitespace() || cmap.map(*c).is_some())
    });
    if covered { primary.clone() } else { thai_face().clone() }
}

/// Shaped glyphs of one run in font units at 1000 units per em: (glyph id, x, y) and the advance.
struct Placed {
    glyphs: Vec<(u32, f64, f64)>,
    advance: f64,
}

/// Shaped runs keyed by (font bytes address, face index, text); faces live for the whole process.
type RunCache = HashMap<(usize, u32, String), Arc<Placed>>;

fn run_cache() -> &'static Mutex<RunCache> {
    static C: OnceLock<Mutex<RunCache>> = OnceLock::new();
    C.get_or_init(|| Mutex::new(HashMap::new()))
}

const MAX_CACHED_RUNS: usize = 10_000;
/// Longer runs are not shaped (input-derived text is hostile).
const MAX_RUN_CHARS: usize = 4_096;

fn placed(face: &Face, run: &[(char, bool, bool)]) -> Option<Arc<Placed>> {
    if run.len() > MAX_RUN_CHARS {
        return None;
    }
    let s: String = run.iter().map(|c| c.0).collect();
    let key = (face.bytes.as_ptr() as usize, face.index, s);
    if let Some(p) = run_cache().lock().unwrap_or_else(PoisonError::into_inner).get(&key) {
        return Some(p.clone());
    }
    let upem = face.font()?.metrics(Size::unscaled(), LocationRef::default()).units_per_em;
    let k = f64::from(UPEM) / f64::from(upem.max(16));
    let hf = harfrust::FontRef::from_index(&face.bytes, face.index).ok()?;
    let data = harfrust::ShaperData::new(&hf);
    let shaper = data.shaper(&hf).build();
    let mut buffer = harfrust::UnicodeBuffer::new();
    buffer.push_str(&key.2);
    buffer.guess_segment_properties();
    let out = shaper.shape(buffer, harfrust::ShapeOptions::new());
    let mut glyphs = Vec::with_capacity(out.glyph_infos().len());
    let mut pen = 0.0;
    for (info, pos) in out.glyph_infos().iter().zip(out.glyph_positions()) {
        glyphs.push((info.glyph_id, pen + f64::from(pos.x_offset) * k, f64::from(pos.y_offset) * k));
        pen += f64::from(pos.x_advance) * k;
    }
    let p = Arc::new(Placed { glyphs, advance: pen });
    let mut cache = run_cache().lock().unwrap_or_else(PoisonError::into_inner);
    if cache.len() > MAX_CACHED_RUNS {
        cache.clear();
    }
    cache.insert(key, p.clone());
    Some(p)
}

/// A shaped Thai run in text units: closed glyph contours grouped per glyph, and its width.
pub(crate) struct ThaiRun {
    pub glyphs: Vec<Vec<Vec<Vec2>>>,
    pub width: f64,
}

/// Shape `run` in `face` at `height` (the face's cap height), starting at `x0` on the baseline.
pub(crate) fn shape(face: &Face, run: &[(char, bool, bool)], height: f64, width_factor: f64, shear: f64, x0: f64) -> Option<ThaiRun> {
    let scale = height / cap_height(&face.font()?);
    let p = placed(face, run)?;
    let mut glyphs = Vec::with_capacity(p.glyphs.len());
    for (gid, gx, gy) in &p.glyphs {
        let g = glyph_by_id(face, GlyphId::new(*gid));
        let contours: Vec<Vec<Vec2>> = g
            .contours
            .iter()
            .map(|ct| {
                ct.iter()
                    .map(|q| {
                        let y = (q.y + gy) * scale;
                        Vec2::new(x0 + (q.x + gx) * scale * width_factor + y * shear, y)
                    })
                    .collect()
            })
            .collect();
        if !contours.is_empty() {
            glyphs.push(contours);
        }
    }
    Some(ThaiRun { glyphs, width: p.advance * scale * width_factor })
}

/// Width of `run` shaped in `face` at `height`.
pub(crate) fn width(face: &Face, run: &[(char, bool, bool)], height: f64, width_factor: f64) -> Option<f64> {
    let cap = cap_height(&face.font()?);
    Some(placed(face, run)?.advance * height / cap * width_factor)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(s: &str) -> Vec<(char, bool, bool)> {
        crate::decode_controls(s)
    }

    #[test]
    fn segments_split_thai_from_latin() {
        let chars = run("ห้อง A-101 ชั้น 2");
        let runs: Vec<(bool, String)> = segments(&chars).map(|(t, r)| (t, r.iter().map(|c| c.0).collect())).collect();
        assert_eq!(runs, [(true, "ห้อง".into()), (false, " A-101 ".into()), (true, "ชั้น".into()), (false, " 2".into())]);
    }

    #[test]
    fn marks_stack_on_their_consonant() {
        // น + ี + ้: the vowel and the tone mark take no width of their own.
        let plain = width(thai_face(), &run("น"), 1.0, 1.0).unwrap_or_default();
        let marked = width(thai_face(), &run("นี้"), 1.0, 1.0).unwrap_or_default();
        assert!(plain > 0.3 && (marked - plain).abs() < 1e-9, "{plain} vs {marked}");
        let shaped = shape(thai_face(), &run("นี้"), 1.0, 1.0, 0.0, 0.0).unwrap_or(ThaiRun { glyphs: Vec::new(), width: 0.0 });
        assert_eq!(shaped.glyphs.len(), 3);
        // The tone mark sits above the upper vowel, not on top of it.
        let top = |g: &Vec<Vec<Vec2>>| g.iter().flatten().map(|p| p.y).fold(f64::MIN, f64::max);
        let bottom = |g: &Vec<Vec<Vec2>>| g.iter().flatten().map(|p| p.y).fold(f64::MAX, f64::min);
        assert!(bottom(&shaped.glyphs[2]) > top(&shaped.glyphs[0]), "tone mark above the consonant");
        assert!(bottom(&shaped.glyphs[2]) >= top(&shaped.glyphs[1]) - 0.05, "tone mark above the vowel");
    }

    #[test]
    fn fonts_without_thai_fall_back() {
        let not_a_font = Face { bytes: Arc::new(b"not a font".to_vec()), index: 0 };
        assert_eq!(face(&not_a_font, &run("ไทย")).bytes.as_ptr(), thai_face().bytes.as_ptr());
        assert_eq!(face(thai_face(), &run("ไทย")).bytes.as_ptr(), thai_face().bytes.as_ptr());
    }
}
