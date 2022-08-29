use std::sync::Arc;

use crate::gen::Bitmap;

pub struct Font<'a> {
    inner: Arc<owned_ttf_parser::Face<'a>>,
}

impl<'a> Font<'a> {
    pub fn from_slice(data: &'a [u8]) -> Self {
        let face = Arc::new(owned_ttf_parser::Face::from_slice(data, 0).unwrap());
        Self { inner: face }
    }

    pub fn glyph_count(&self) -> u16 {
        self.inner.number_of_glyphs()
    }

    pub fn units_per_em(&self) -> u16 {
        self.inner.units_per_em()
    }

    pub fn v_metrics(&self, scale: Scale) -> VMetrics {
        let glyph_height =
            self.inner.ascender() as f32 - self.inner.descender() as f32;
        let height_factor = scale.0 / glyph_height;

        self.v_metrics_unscaled() * height_factor
    }

    pub fn v_metrics_unscaled(&self) -> VMetrics {
        let font = &self.inner;
        VMetrics {
            ascent: font.ascender() as f32,
            descent: font.descender() as f32,
            line_gap: font.line_gap() as f32,
        }
    }

    pub fn glyph<C: Into<GlyphId>>(&self, id: C) -> Glyph<'a> {
        let code_point = id.into();
        let font = Arc::clone(&self.inner);
        assert!(code_point.0 < self.glyph_count());

        Glyph {
            font,
            id: code_point,
        }
    }
}

pub struct Glyph<'font> {
    font: Arc<owned_ttf_parser::Face<'font>>,
    id: GlyphId,
}

impl Glyph<'_> {
    pub fn generate_sdf(&self) -> Bitmap {
        crate::gen::generate_sdf(todo!(), todo!());
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Scale(f32);

pub struct VMetrics {
    pub ascent: f32,
    pub descent: f32,
    pub line_gap: f32,
}

impl std::ops::Mul<f32> for VMetrics {
    type Output = VMetrics;

    fn mul(self, rhs: f32) -> Self::Output {
        VMetrics {
            ascent: self.ascent * rhs,
            descent: self.descent * rhs,
            line_gap: self.line_gap * rhs,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct GlyphId(pub u16);

impl From<char> for GlyphId {
    fn from(c: char) -> Self {
        GlyphId(c as u16)
    }
}
