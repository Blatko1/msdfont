mod font;
mod gen;
mod math;
mod overlaps;
mod shape;
mod vector;

use image::{DynamicImage, GenericImage, Rgba};
use rusttype::{Font, Point, Scale};
use shape::ShapeBuilder;

use crate::{math::SignedDistance, shape::Segment, vector::Vector2};

const DEFAULT_SCALE: Scale = Scale { x: 64.0, y: 64.0 };

pub struct Msdfont<'a> {
    font: Font<'a>,

    scale: Scale,
}

impl<'a> Msdfont<'a> {
    pub fn try_from_bytes(font: &'a [u8]) -> Result<Self, ()> {
        let f = Font::try_from_bytes(font).ok_or(())?;
        Ok(Self::from_font(f))
    }

    pub fn from_font(font: Font<'a>) -> Self {
        Self::new(font)
    }

    fn new(font: Font<'a>) -> Self {
        Self {
            font,
            scale: DEFAULT_SCALE,
        }
    }

    pub fn scale(&mut self, scale: f32) {
        self.scale = Scale::uniform(scale);
    }

    /*pub fn generate_sdf(&self, c: char) {
        let glyph = self
            .font
            .glyph(c)
            .scaled(self.scale)
            .positioned(Point { x: 0.0, y: 0.0 });

        let bb = glyph.pixel_bounding_box().unwrap();
        let pos = glyph.position();

        let mut builder = ShapeBuilder::new((pos.x, pos.y));

        glyph.build_outline(&mut builder);

        let shape = builder.build();

        let width = bb.width() as u32;
        let height = bb.height() as u32;

        for y in 0..height {
            for x in 0..width {
                let point = Vector2::new(x as f32 + 0.5, y as f32 + 0.5);

                let mut sgn_distance = SignedDistance::MAX;

                for contour in shape.iter() {
                    for seg in &contour.segments {
                        let sd = match seg {
                            Segment::Line(line) => Some(line.calculate_sd(point)),
                            Segment::Quadratic(quad) => {
                                Some(quad.calculate_sd(point))
                            }
                            Segment::Cubic(curve) => None,
                            Segment::End() => None,
                        };

                        if let Some(sd) = sd {
                            if sd > sgn_distance {
                                continue;
                            }
                            sgn_distance = sd;
                        }
                    }
                }

                // Pixel color
                const MAX_DIST: f32 = 6.0;
                // Used for normal SDF
                let real_d =
                    ((sgn_distance.sign * sgn_distance.real_dist / MAX_DIST) + 0.5)
                        .clamp(0.0, 1.0);
                // Used for pseudo-SDF
                let extended_d = ((sgn_distance.sign * sgn_distance.extended_dist
                    / MAX_DIST)
                    + 0.5)
                    .clamp(0.0, 1.0);

                let sdf = (extended_d * 255.0) as u8;
            }
        }
    }*/
}

#[test]
fn main_test() {
    let data = include_bytes!("../examples/fonts/monserat.ttf");
    let font = Font::try_from_bytes(data).unwrap();

    let scale = Scale::uniform(300.0);
    let offset = 0;

    let char = 'Đ';

    let glyph = font.glyph(char).scaled(scale).positioned(Point {
        x: offset as f32 / 2.0,
        y: offset as f32 / 2.0,
    });

    let bb = glyph.pixel_bounding_box().unwrap();
    let pos = glyph.position();

    let mut builder = ShapeBuilder::new(Vector2::new(pos.x, pos.y));

    glyph.build_outline(&mut builder);

    let shape = builder.build();

    let width = bb.width() as u32;
    let height = bb.height() as u32;

    let mut image = DynamicImage::new_rgb8(width + offset, height + offset);

    for y in 0..height {
        for x in 0..width {
            //let x = 76;
            //let y = 67;
            let pixel = Vector2::new(x as f32 + 0.5, y as f32 + 0.5);

            let distance = gen::pixel_distance(&shape, pixel);
            //println!("final: {:?}", distance);
            let signed_distance = distance.sign * distance.real_dist;

            // Pixel color
            const MAX_DIST: f32 = 12.0;

            // Used for normal SDF
            let normalized = (signed_distance / MAX_DIST as f32) + 0.5;
            // Used for pseudo-SDF
            //let pseudo = ((distance.sign * distance.extended_dist
            //    / pxrange)
            //    + 0.5)
            //    .clamp(0.0, 1.0);

            // When f32 is being converted to u8 it is automatically
            // clamped in range [0, 255].
            let sdf = (normalized * 255.0) as u8;

            image.put_pixel(x, y, Rgba([sdf, sdf, sdf, 255]));
        }
    }
    image.save("examples/test.png").unwrap();
}
