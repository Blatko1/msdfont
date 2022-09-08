use std::{sync::Arc, ops::Mul};

use rusttype::{Font as RTFont, GlyphId, Glyph as RTGlyph};

use crate::{
    gen::Bitmap,
    path::{PathBuilder},
    vector::Vector2, shape::Shape,
};

pub struct Font<'a> {
    inner: Arc<RTFont<'a>>,
}

impl<'a> Font<'a> {
    pub fn from_slice(data: &'a [u8]) -> Self {
        // TODO add custom errors for results
        let face = Arc::new(RTFont::try_from_bytes(data).unwrap());
        Self { inner: face }
    }

    pub fn glyph_count(&self) -> usize {
        self.inner.glyph_count()
    }

    pub fn units_per_em(&self) -> u16 {
        self.inner.units_per_em()
    }

    pub fn v_metrics(&self, scale: Scale) -> VMetrics {
        self.inner.v_metrics(scale)
    }

    pub fn v_metrics_unscaled(&self) -> VMetrics {
        self.inner.v_metrics_unscaled()
    }

    // TODO maybe use IntoGlyphId
    pub fn glyph<C: Into<char>>(&self, id: C) -> Glyph<'a> {
        let glyph = self.inner.glyph(id);
        let font = Arc::clone(&self.inner);
        //assert!(index.0 < self.glyph_count());

        Glyph { inner: glyph, units_per_em: self.units_per_em() }
    }
}

pub struct Glyph<'font> {
    inner: RTGlyph<'font>,
    units_per_em: u16,
}

impl Glyph<'_> {
    pub fn build(&self, scale: Scale) -> GlyphOutline {
        let norm_scale = scale.normalize(self.units_per_em as f32);
        let mut builder = PathBuilder::new_with_norm_scale(norm_scale);

        let unscaled_rect = self.inner.scaled(scale)outline_glyph(self.id, &mut builder).unwrap();

        let mut bbox = BBox::from(unscaled_rect);
        bbox._resize(norm_scale);
        dbg!(bbox);

        let shape = builder.build_shape();

        GlyphOutline { bbox, shape }
    }
}

pub struct GlyphOutline {
    pub(crate) bbox: BBox,
    pub(crate) shape: Shape,
}

impl GlyphOutline {

    pub fn from_shape(shape: Shape, bbox: BBox) -> Self {
        Self { bbox, shape }
    }

    /// Consumes the [`Glyph`] and returns a image bitmap with
    /// signed distance fields.
    pub fn generate_sdf(self, range: usize) -> Bitmap {
        crate::gen::gen_sdf(self, range)
    }

    /// Consumes the [`Glyph`] and returns a image bitmap with
    /// pseudo signed distance fields.
    pub fn generate_pseudo_sdf(self, range: usize) -> Bitmap {
        crate::gen::gen_pseudo_sdf(self, range)
    }

    #[inline]
    pub fn width(&self) -> f32 {
        self.bbox.width()
    }

    #[inline]
    pub fn height(&self) -> f32 {
        self.bbox.height()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct BBox {
    /// Top left point.
    pub tl: Vector2,
    /// Bottom right point.
    pub br: Vector2,
}

impl BBox {
    pub fn new(tl: Vector2, br: Vector2) -> Self {
        Self {
            tl,
            br,
        }
    }

    pub fn resize(&mut self, scale: Scale) {
        let scale = scale.normalize(1.0);
        self._resize(scale);
    }

    pub(crate) fn _resize(&mut self, scale: NormScale) {
        self.tl = self.tl * scale;
        self.br = self.br * scale;
    }

    #[inline]
    pub fn width(&self) -> f32 {
        self.br.x - self.tl.x
    }

    #[inline]
    pub fn height(&self) -> f32 {
        self.tl.y - self.br.y
    }
}

impl From<Rect> for BBox {
    fn from(rect: Rect) -> Self {
        BBox {
            tl: Vector2::new(rect.x_min as f32, rect.y_max as f32),

            br: Vector2::new(rect.x_max as f32, rect.y_min as f32),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Scale(pub f32);

impl Scale {
    pub fn new(scale: f32) -> Self {
        Self(scale)
    }

    pub(crate) fn normalize(self, div_factor: f32) -> NormScale {
        NormScale(self.0 / div_factor)
    }
}

impl Default for Scale {
    fn default() -> Self {
        Self(1.0)
    }
}

/// Represents a normalized scale which is used for
/// scaling the glyph in the build process.
/// 
/// Normalized means that the scale has already been divided by
/// the units_per_em factor which each font has.
/// // TODO should scale field be public?
#[derive(Debug, Clone, Copy)]
pub struct NormScale(f32);

impl Mul<f32> for NormScale {
    type Output = f32;

    fn mul(self, rhs: f32) -> Self::Output {
        self.0 * rhs
    }
}

impl Mul<NormScale> for f32 {
    type Output = f32;

    fn mul(self, rhs: NormScale) -> Self::Output {
        self * rhs.0
    }
}

impl Mul<Vector2> for NormScale {
    type Output = Vector2;

    fn mul(self, rhs: Vector2) -> Self::Output {
        rhs * self.0
    }
}

impl Mul<NormScale> for Vector2 {
    type Output = Vector2;

    fn mul(self, rhs: NormScale) -> Self::Output {
        self * rhs.0
    }
}

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

//#[derive(Debug, Clone, Copy)]
//pub struct GlyphId(pub u16);
//
//impl From<char> for GlyphId {
//    fn from(c: char) -> Self {
//        GlyphId(c as u16)
//    }
//}
//
//impl Into<owned_ttf_parser::GlyphId> for GlyphId {
//    fn into(self) -> owned_ttf_parser::GlyphId {
//        owned_ttf_parser::GlyphId(self.0)
//    }
//}
