mod contour;
mod font;
mod gen;
mod math;
mod overlaps;
mod shape;
mod vector;

use font::{Font, Scale};
use image::{DynamicImage, GenericImage, Rgba};
use rusttype::Point;
use shape::ShapeBuilder;

use crate::vector::Vector2;

#[test]
fn main_test() {
    let data = include_bytes!("../examples/fonts/monserat.ttf");
    //let font = Font::try_from_bytes(data).unwrap();
    let font = Font::from_slice(data);

    let scale = Scale(80.0 / font.units_per_em() as f32);
    let offset = 5;

    let char = 'Đ';

    let glyph = font.glyph(char).build(scale);
    //.scaled(scale).positioned(Point {
    //    x: offset as f32 / 2.0,
    //    y: offset as f32 / 2.0,
    //});
    //let gen = glyph.generate_sdf().data();
    println!("here");
    let bb = glyph.bbox;

    let width = bb.width() as u32;
    let height = bb.height() as u32;

    let mut image = DynamicImage::new_rgb8(width + offset, height + offset);

    //for y in 0..height {
    //    for x in 0..width {
    //        let pixel = gen[(y * width + x) as usize];
    //        image.put_pixel(x, y, Rgba([pixel, pixel, pixel, 255]));
    //    }
    //}
    for y in 0..height + offset {
        for x in 0..width + offset {
            //let x = 76;
            //let y = 67;
            let pixel = Vector2::new(x as f32 + 0.5, y as f32 + 0.5);

            let distance = gen::pixel_distance(&glyph.shape, pixel);
            //println!("final: {:?}", distance);
            let signed_distance = -distance.sign * distance.real_dist;

            // Pixel color
            const MAX_DIST: f32 = 6.0;

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

            image.put_pixel(x, height + offset - y - 1, Rgba([sdf, sdf, sdf, 255]));
        }
    }
    image.save("examples/test.png").unwrap();
}
