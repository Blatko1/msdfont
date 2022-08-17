mod math;
mod shape;

use cgmath::Vector2;
use image::{DynamicImage, GenericImage, Rgba};
use rusttype::{Font, Point, Scale};

use crate::{
    math::SignedDistance,
    shape::{Quad, Segment, ShapeBuilder},
};

fn main() {
    let data = include_bytes!("monserat.ttf");
    let font = Font::try_from_bytes(data).unwrap();

    let scale = Scale::uniform(100.0);
    let offset = 20;

    let char = 'Đ';

    let glyph = font.glyph(char).scaled(scale).positioned(Point {
        x: offset as f32 / 2.0,
        y: offset as f32 / 2.0,
    });

    let bb = glyph.pixel_bounding_box().unwrap();

    let mut builder = ShapeBuilder::new(glyph.position());

    glyph.build_outline(&mut builder);

    let shape = builder.build();

    let width = bb.width() as u32;
    let height = bb.height() as u32;

    let mut image = DynamicImage::new_rgba8(width + offset, height + offset);

    for y in 0..height + offset {
        for x in 0..width + offset {
            let point = Vector2::new(x as f32 + 0.5, y as f32 + 0.5);

            let mut sgn_distance = SignedDistance::MAX;

            for contour in shape.get_segments() {
                for seg in &contour.segments {
                    let sd = match seg {
                        Segment::Line(line) => Some(line.calculate_sd(point)),
                        Segment::Quadratic(quad) => Some(quad.calculate_sd(point)),
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
            /*let curve = Quad {
                from: Vector2 { x: 100.5726, y: 10.58819 },
                control: Vector2 { x: 10.5726, y: 75.58819 },
                to: Vector2 { x: 160.56276, y: 160.80722 },
            };
            sgn_distance = curve.calculate_sd(point);*/

            // Pixel color
            const MAX_DIST: f32 = 6.0;
            // Used for normal SDF
            let real_d = ((sgn_distance.sign * sgn_distance.real_dist / MAX_DIST)
                + 0.5)
                .clamp(0.0, 1.0);
            // Used for pseudo-SDF
            let extended_d =
                ((sgn_distance.sign * sgn_distance.extended_dist / MAX_DIST) + 0.5)
                    .clamp(0.0, 1.0);

            let sdf = (extended_d * 255.0) as u8;

            image.put_pixel(x, y, Rgba([sdf, sdf, sdf, 255]));
        }
    }
    image.save("output/test.png").unwrap();
}
