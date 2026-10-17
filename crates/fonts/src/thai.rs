//! Thai text in drawings (CadKub). The stroke font has no Thai letters, and Thai needs shaping:
//! upper and lower vowels and tone marks sit on their consonant, not after it. So lines with Thai
//! take the complex-script path ([`crate::complex`]), which shapes each font piece with harfrust,
//! and Thai letters the text's font lacks are drawn in the bundled Sarabun rather than whatever
//! fallback font a machine happens to have, so drawings look the same everywhere.

use std::sync::{Arc, OnceLock};

use crate::ttf::Face;

/// The bundled Thai face (Sarabun Regular, SIL Open Font License 1.1).
pub static THAI_FONT: &[u8] = include_bytes!("../../../assets/fonts/Sarabun-Regular.ttf");

/// [`THAI_FONT`] as a face. One copy for the whole process: the glyph caches key faces by address.
pub fn thai_face() -> Option<&'static Face> {
    static F: OnceLock<Option<Face>> = OnceLock::new();
    F.get_or_init(|| Face::new(Arc::new(THAI_FONT.to_vec()), 0)).as_ref()
}

/// Thai block letters, vowels, tone marks and digits.
pub fn is_thai(c: char) -> bool {
    ('\u{0E00}'..='\u{0E7F}').contains(&c)
}

/// Lines laid out by the complex-script path: right-to-left text, or Thai.
pub(crate) fn needs_shaping(s: &str) -> bool {
    crate::bidi::has_rtl(s) || s.chars().any(is_thai)
}

#[cfg(test)]
mod tests {
    use crate::{TextFont, layout_line, shape_line, text_width};

    #[test]
    fn marks_stack_on_their_consonant() {
        // น + ี + ้: the vowel and the tone mark take no width of their own.
        let font = TextFont::Stroke;
        let plain = text_width(&font, "น", 1.0, 1.0);
        let marked = text_width(&font, "นี้", 1.0, 1.0);
        assert!(plain > 0.3 && (marked - plain).abs() < 1e-9, "{plain} vs {marked}");
        let sh = shape_line(&font, "นี้", 1.0, 1.0, 0.0);
        assert_eq!(sh.glyphs.len(), 3);
        let top = |g: &Vec<Vec<cadcraft_geom::Vec2>>| g.iter().flatten().map(|p| p.y).fold(f64::MIN, f64::max);
        let bottom = |g: &Vec<Vec<cadcraft_geom::Vec2>>| g.iter().flatten().map(|p| p.y).fold(f64::MAX, f64::min);
        assert!(bottom(&sh.glyphs[2]) > top(&sh.glyphs[0]), "tone mark above the consonant");
        assert!(bottom(&sh.glyphs[2]) >= top(&sh.glyphs[1]) - 0.05, "tone mark above the vowel");
    }

    #[test]
    fn thai_in_the_stroke_font_uses_the_thai_face() {
        let font = TextFont::Stroke;
        let sh = shape_line(&font, "ห้อง A-101", 1.0, 1.0, 0.0);
        assert_eq!(sh.glyphs.len(), 4, "ห ้ อ ง as outlines");
        assert!(!sh.strokes.is_empty(), "the Latin part stays in the stroke font");
        assert!((text_width(&font, "ห้อง A-101", 1.0, 1.0) - sh.width).abs() < 1e-9);
        // The Latin part starts after the Thai word.
        let thai_right = sh.glyphs.iter().flatten().flatten().map(|p| p.x).fold(f64::MIN, f64::max);
        let latin_left = sh.strokes.iter().flatten().map(|p| p.x).fold(f64::MAX, f64::min);
        assert!(latin_left > thai_right, "{latin_left} > {thai_right}");
        // Lines without Thai are unchanged.
        assert_eq!(shape_line(&font, "A-101", 1.0, 1.0, 0.0).strokes, layout_line("A-101", 1.0, 1.0, 0.0).strokes);
    }

    #[test]
    fn thai_in_a_font_without_thai_uses_the_thai_face() {
        // Skips cleanly when none of these Latin-only system fonts is installed.
        let Some(face) = ["Arial", "Helvetica", "DejaVuSans", "Verdana"].iter().find_map(|n| crate::ttf::find(n)) else {
            return;
        };
        if face.has_glyph('ไ') {
            return; // this system's copy has Thai letters of its own
        }
        let sh = crate::ttf::shape(&face, "ไทย AB", 1.0, 1.0, 0.0).unwrap_or_default();
        assert_eq!(sh.glyphs.len(), 5, "three Thai letters from the Thai face, then A and B");
        assert!((crate::ttf::width(&face, "ไทย AB", 1.0, 1.0).unwrap_or_default() - sh.width).abs() < 1e-9);
    }
}
