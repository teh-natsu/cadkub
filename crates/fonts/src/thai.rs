//! Thai text in drawings (CadKub). The stroke font has no Thai letters and most drafting
//! TrueType fonts have none either, so a Thai run is set in the style's own font when it covers
//! the run and in the bundled Sarabun otherwise. Every Thai run is shaped with harfrust, so
//! upper and lower vowels and tone marks sit on their consonant instead of after it.

use std::collections::HashMap;
use std::sync::{Arc, Mutex, OnceLock, PoisonError};

use cadcraft_geom::Vec2;
use skrifa::instance::{LocationRef, Size};
use skrifa::{FontRef, GlyphId, MetadataProvider};

use crate::ttf::{UPEM, cap_height, glyph_by_id};

/// The bundled Thai face (Sarabun Regular, SIL Open Font License 1.1). A static, not a const:
/// the glyph caches key fonts by address.
pub static THAI_FONT: &[u8] = include_bytes!("../../../assets/fonts/Sarabun-Regular.ttf");

/// Thai block letters, vowels, tone marks and digits.
pub fn is_thai(c: char) -> bool {
    ('\u{0E00}'..='\u{0E7F}').contains(&c)
}

/// Decoded characters split into alternating non-Thai and Thai runs: `(is_thai, run)`.
pub(crate) fn segments(chars: &[(char, bool, bool)]) -> impl Iterator<Item = (bool, &[(char, bool, bool)])> {
    chars.chunk_by(|a, b| is_thai(a.0) == is_thai(b.0)).map(|run| (run.first().is_some_and(|c| is_thai(c.0)), run))
}

/// The face for a Thai run: `primary` when it maps every letter of `s`, else the bundled one.
pub(crate) fn face<'a>(primary: &'a [u8], s: &str) -> &'a [u8] {
    let covered = FontRef::new(primary).is_ok_and(|f| {
        let cmap = f.charmap();
        s.chars().all(|c| c.is_whitespace() || cmap.map(c).is_some())
    });
    if covered { primary } else { THAI_FONT }
}

/// Shaped glyphs of one run in font units at 1000 units per em: (glyph id, x, y) and the advance.
struct Placed {
    glyphs: Vec<(u32, f64, f64)>,
    advance: f64,
}

/// Shaped runs keyed by (font bytes address, length, text); fonts live for the whole process.
type RunCache = HashMap<(usize, usize, String), Arc<Placed>>;

fn run_cache() -> &'static Mutex<RunCache> {
    static C: OnceLock<Mutex<RunCache>> = OnceLock::new();
    C.get_or_init(|| Mutex::new(HashMap::new()))
}

const MAX_CACHED_RUNS: usize = 10_000;
/// Longer runs are not shaped (input-derived text is hostile).
const MAX_RUN_CHARS: usize = 4_096;

fn placed(font: &[u8], s: &str) -> Option<Arc<Placed>> {
    let key = (font.as_ptr() as usize, font.len(), s.to_string());
    if let Some(p) = run_cache().lock().unwrap_or_else(PoisonError::into_inner).get(&key) {
        return Some(p.clone());
    }
    if s.chars().count() > MAX_RUN_CHARS {
        return None;
    }
    let upem = FontRef::new(font).ok()?.metrics(Size::unscaled(), LocationRef::default()).units_per_em;
    let k = f64::from(UPEM) / f64::from(upem.max(16));
    let hf = harfrust::FontRef::new(font).ok()?;
    let data = harfrust::ShaperData::new(&hf);
    let shaper = data.shaper(&hf).build();
    let mut buffer = harfrust::UnicodeBuffer::new();
    buffer.push_str(s);
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

/// Shape `s` in `font` at `height` (the font's cap height), starting at `x0` on the baseline.
pub(crate) fn shape(font: &[u8], s: &str, height: f64, width_factor: f64, shear: f64, x0: f64) -> Option<ThaiRun> {
    let f = FontRef::new(font).ok()?;
    let p = placed(font, s)?;
    let scale = height / cap_height(&f);
    let mut glyphs = Vec::with_capacity(p.glyphs.len());
    for (gid, gx, gy) in &p.glyphs {
        let g = glyph_by_id(font, &f, GlyphId::new(*gid));
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

/// Width of `s` shaped in `font` at `height`.
pub(crate) fn width(font: &[u8], s: &str, height: f64, width_factor: f64) -> Option<f64> {
    let f = FontRef::new(font).ok()?;
    Some(placed(font, s)?.advance * height / cap_height(&f) * width_factor)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn segments_split_thai_from_latin() {
        let chars = crate::decode_controls("ห้อง A-101 ชั้น 2");
        let runs: Vec<(bool, String)> = segments(&chars).map(|(t, r)| (t, r.iter().map(|c| c.0).collect())).collect();
        assert_eq!(runs, [(true, "ห้อง".into()), (false, " A-101 ".into()), (true, "ชั้น".into()), (false, " 2".into())]);
    }

    #[test]
    fn marks_stack_on_their_consonant() {
        // น + ี + ้: the vowel and the tone mark take no width of their own.
        let plain = width(THAI_FONT, "น", 1.0, 1.0).unwrap_or_default();
        let marked = width(THAI_FONT, "นี้", 1.0, 1.0).unwrap_or_default();
        assert!(plain > 0.3 && (marked - plain).abs() < 1e-9, "{plain} vs {marked}");
        let run = shape(THAI_FONT, "นี้", 1.0, 1.0, 0.0, 0.0).unwrap_or(ThaiRun { glyphs: Vec::new(), width: 0.0 });
        assert_eq!(run.glyphs.len(), 3);
        // The tone mark sits above the upper vowel, not on top of it.
        let top = |g: &Vec<Vec<Vec2>>| g.iter().flatten().map(|p| p.y).fold(f64::MIN, f64::max);
        let bottom = |g: &Vec<Vec<Vec2>>| g.iter().flatten().map(|p| p.y).fold(f64::MAX, f64::min);
        assert!(bottom(&run.glyphs[2]) > top(&run.glyphs[0]), "tone mark above the consonant");
        assert!(bottom(&run.glyphs[2]) >= top(&run.glyphs[1]) - 0.05, "tone mark above the vowel");
    }

    #[test]
    fn fonts_without_thai_fall_back() {
        assert_eq!(face(b"not a font", "ไทย").as_ptr(), THAI_FONT.as_ptr());
        assert_eq!(face(THAI_FONT, "ไทย").as_ptr(), THAI_FONT.as_ptr());
    }
}
