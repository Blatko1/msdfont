use std::{sync::Arc, ops::Mul};

use owned_ttf_parser::{GlyphId, Rect};

use crate::{
    gen::Bitmap,
    path::{PathBuilder},
    vector::Vector2, shape::Shape,
};

pub struct Font<'a> {
    inner: Arc<owned_ttf_parser::Face<'a>>,
}

impl<'a> Font<'a> {
    pub fn from_slice(data: &'a [u8]) -> Self {
        // TODO add custom errors for results
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
        let scale = scale.normalize(self.units_per_em() as f32);
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

    pub fn glyph<C: Into<char>>(&self, id: C) -> Glyph<'a> {
        let index = self.inner.glyph_index(id.into()).unwrap();
        let font = Arc::clone(&self.inner);
        //assert!(index.0 < self.glyph_count());

        Glyph { font, units_per_em: self.units_per_em(), id: index }
    }
}

pub struct Glyph<'font> {
    font: Arc<owned_ttf_parser::Face<'font>>,
    units_per_em: u16,
    id: GlyphId,
}

impl Glyph<'_> {
    pub fn build(&self, scale: Scale) -> GlyphOutline {
        let norm_scale = scale.normalize(self.units_per_em as f32);
        let mut builder = PathBuilder::new_with_norm_scale(norm_scale);

        let unscaled_rect = self.font.outline_glyph(self.id, &mut builder).unwrap();

        let bbox = BBox::from(unscaled_rect).resize(norm_scale);
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

    pub fn from_shape(shape: Shape) -> Self {
        let bbox = shape.bbox();
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
    fn resize(self, scale: NormScale) -> BBox {
        BBox {
            tl: self.tl * scale,
            br: self.br * scale,
        }
    }

    #[inline]
    fn width(&self) -> f32 {
        self.br.x - self.tl.x
    }

    #[inline]
    fn height(&self) -> f32 {
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
