use std::sync::Arc;

use rusttype::{Font as RTFont, Glyph as RTGlyph, Rect, Scale, VMetrics};

use crate::{gen::Bitmap, path::PathBuilder, shape::Shape, vector::Vector2};

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
        self.inner.v_metrics(scale.into())
    }

    pub fn v_metrics_unscaled(&self) -> VMetrics {
        self.inner.v_metrics_unscaled()
    }

    // TODO maybe use IntoGlyphId
    pub fn glyph<C: Into<char>>(&self, id: C) -> Glyph<'a> {
        let glyph = self.inner.glyph(id.into());
        // let font = Arc::clone(&self.inner);

        Glyph { inner: glyph }
    }
}

pub struct Glyph<'font> {
    inner: RTGlyph<'font>,
}

impl Glyph<'_> {
    /// The glyph identifier for this glyph.
    pub fn id(&self) -> u16 {
        self.inner.id().0
    }

    /// Builds a [`GlyphOutline`] with the provided [`Scale`] and [`Offset`].
    ///
    /// Scale is automatically normalized by the `units_per_em` factor.
    ///
    /// Offset is mainly used in the sdf generation process for better view
    /// of the glyph. It adds empty space to the left, right, top or bottom
    /// of the outline .
    pub fn build(self, scale: Scale, offset: Offset) -> GlyphOutline {
        // Offset the shape to the right and the bottom
        let pos = rusttype::Point {
            x: offset.x,
            y: offset.y,
        };
        let glyph = self.inner.scaled(scale.into()).positioned(pos);
        let mut builder = PathBuilder::new(offset);
        
        let bbox = BBox::from(glyph.pixel_bounding_box().unwrap());
        dbg!(bbox);

        let result = glyph.build_outline(&mut builder);
        assert!(result, "Glyph outline error!");

        let shape = builder.build_shape();

        GlyphOutline::from_shape(shape, bbox, offset)
    }
}

pub struct GlyphOutline {
    pub(crate) bbox: BBox,
    pub(crate) shape: Shape,
    pub(crate) offset: Offset
}

impl GlyphOutline {
    /// Initialize a new [`GlyphOutline`] with the provided shape and it's
    /// bounding box.
    ///
    /// Use the [`Self::generate`] functions to create a distance field bitmap.
    pub fn from_shape(shape: Shape, bbox: BBox, offset: Offset) -> Self {
        Self { bbox, shape, offset }
    }

    /// Returns a image bitmap with signed distance fields.
    pub fn generate_sdf(&self, range: usize) -> Bitmap {
        crate::gen::gen_sdf(self, range)
    }

    /// Returns a image bitmap with pseudo signed distance fields.
    pub fn generate_pseudo_sdf(&self, range: usize) -> Bitmap {
        crate::gen::gen_pseudo_sdf(self, range)
    }

    /// Returns the width of the shape's bounding box.
    #[inline]
    pub fn width(&self) -> i32 {
        self.bbox.width()
    }

    /// Returns the height of the shape's bounding box.
    #[inline]
    pub fn height(&self) -> i32 {
        self.bbox.height()
    }
}

/// `Bounding box` represents an imaginary rectangle.
///
/// - `tl` - represents the top left point of the rectangle
/// - `br` - represents the top left point of the rectangle
///
/// [`BBox`] implies that the uv coordinate system is used meaning
/// y coordinate increases downwards.
#[derive(Debug, Clone, Copy)]
pub struct BBox {
    /// Top left point.
    pub tl: Vector2<i32>,
    /// Bottom right point.
    pub br: Vector2<i32>,
}

impl BBox {
    pub fn new(tl: Vector2<i32>, br: Vector2<i32>) -> Self {
        Self { tl, br }
    }

    pub fn scale(&mut self, scale: Scale) {
        self.tl.x *= scale.x.ceil() as i32;
        self.tl.y *= scale.y.ceil() as i32;
        self.br.x *= scale.x.ceil() as i32;
        self.br.y *= scale.y.ceil() as i32;
    }

    #[inline]
    pub fn width(&self) -> i32 {
        self.br.x - self.tl.x
    }

    // TODO maybe fix
    #[inline]
    pub fn height(&self) -> i32 {
        // y increases downwards
        self.br.y - self.tl.y
    }
}

impl From<Rect<i32>> for BBox {
    fn from(rect: Rect<i32>) -> Self {
        BBox {
            tl: Vector2::new(rect.min.x, rect.min.y),
            br: Vector2::new(rect.max.x, rect.max.y),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Offset {
    pub x: f32,
    pub y: f32,
}

impl Offset {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
    pub fn uniform(offset: f32) -> Self {
        Self {
            x: offset,
            y: offset,
        }
    }
}

// TODO is needed?? /// Used for scaling glyphs and getting the desired output dimensions.
// #[derive(Debug, Clone, Copy)]
// pub struct Scale {
//     x: f32,
//     y: f32
// }
//
// impl Scale {
//     pub fn new(x: f32, y: f32) -> Self {
//         Self {
//             x,
//             y,
//         }
//     }
//
//     pub fn uniform(scale: f32) -> Self {
//         Self { x: scale, y: scale }
//     }
// }

// impl Into<rusttype::Scale> for Scale {
//     fn into(self) -> rusttype::Scale {
//         rusttype::Scale { x: self.x, y: self.y }
//     }
// }

//// Represents a normalized scale which is used for
//// scaling the glyph in the build process.
////
//// Normalized means that the scale has already been divided by
//// the units_per_em factor which each font has.
//// // TODO should scale field be public?
// #[derive(Debug, Clone, Copy)]
// pub struct NormScale(f32);
//
// impl Mul<f32> for NormScale {
//     type Output = f32;
//
//     fn mul(self, rhs: f32) -> Self::Output {
//         self.0 * rhs
//     }
// }
//
// impl Mul<NormScale> for f32 {
//     type Output = f32;
//
//     fn mul(self, rhs: NormScale) -> Self::Output {
//         self * rhs.0
//     }
// }
//
// impl Mul<Vector2> for NormScale {
//     type Output = Vector2;
//
//     fn mul(self, rhs: Vector2) -> Self::Output {
//         rhs * self.0
//     }
// }
//
// impl Mul<NormScale> for Vector2 {
//     type Output = Vector2;
//
//     fn mul(self, rhs: NormScale) -> Self::Output {
//         self * rhs.0
//     }
// }

// pub struct VMetrics {
//     pub ascent: f32,
//     pub descent: f32,
//     pub line_gap: f32,
// }
//
// impl std::ops::Mul<f32> for VMetrics {
//     type Output = VMetrics;
//
//     fn mul(self, rhs: f32) -> Self::Output {
//         VMetrics {
//             ascent: self.ascent * rhs,
//             descent: self.descent * rhs,
//             line_gap: self.line_gap * rhs,
//         }
//     }
// }

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
