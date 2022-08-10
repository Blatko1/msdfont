mod math;
mod shape;

use cgmath::Vector2;
use image::{DynamicImage, GenericImage, Rgba};
use rusttype::{Font, Point, Scale};

use crate::{shape::{Segment, ShapeBuilder}, math::SignedDistance};

fn main() {
    let data = include_bytes!("monserat.ttf");
    let font = Font::try_from_bytes(data).unwrap();

    let scale = Scale::uniform(100.0);

    let char = 'M';

    let glyph = font
        .glyph(char)
        .scaled(scale)
        .positioned(Point { x: 1.0, y: 1.0 });

    let bb = glyph.pixel_bounding_box().unwrap();

    let mut builder = ShapeBuilder::default();

    glyph.build_outline(&mut builder);

    let shape = builder.build();

    let width = bb.width() as u32;
    let height = bb.height() as u32;

    let mut image = DynamicImage::new_rgba8(width, height);

    for y in 0..height {
        for x in 0..width {

            let p = Vector2::new(x as f32, y as f32);

            let mut sgn_distance = SignedDistance::MAX;

            for contour in shape.get_segments() {
                for seg in &contour.segments {
                    let sd = match seg {
                        Segment::Line(line) => Some(line.calculate_sd(p)),
                        Segment::Quadratic(quad) => None,
                        Segment::Cubic(curve) => None,
                        Segment::End() => None,
                    };
                    if let Some(sd) = sd {
                        // Compare
                        if sd > sgn_distance {
                            continue;
                        }
                        sgn_distance = sd;
                    }
                }
            }
            // Distance color
            const MAX_DIST: f32 = 4.0;
            let d = (sgn_distance.real_dist / MAX_DIST).clamp(-1.0, 1.0) * 0.5 + 0.5;
            let sdf = ((1.0 - d) * 255.0) as u8;
            //println!("closest to: {} {} is line: {:?}", x, y, closest);

            image.put_pixel(x, y, Rgba([sdf, sdf, sdf, 255]));
        }
    }
    image.save("image.png").unwrap();
}

fn is_closer(dist: f32, cmp_with: f32) -> bool {
    if (cmp_with - dist).abs() <= 0.01 {
        false
    } else {
        dist > cmp_with
    }
}
