use image::{DynamicImage, GenericImage, Rgba};
use msdfont::{Font, Scale};

fn main() {
    let data = include_bytes!("fonts/monserat.ttf");

    let font = Font::from_slice(data);

    let glyph = font.glyph('Ž').build(Scale(100.0));
    let width = glyph.width() as u32;
    let height = glyph.height() as u32;
    let offset = 10;

    let sdf = glyph.generate_sdf(6).data();

    let mut image = DynamicImage::new_rgb8(width + offset, height + offset);

    for y in 0..height {
        for x in 0..width {
            let pixel = sdf[((height - y - 1) * width + x) as usize];
            image.put_pixel(x, y, Rgba([pixel, pixel, pixel, 255]));
        }
    }

    image.save("examples/test.png").unwrap();
}
