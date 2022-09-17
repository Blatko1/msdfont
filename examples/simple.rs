use image::{DynamicImage, GenericImage, Rgba};
use msdfont::{Font, Offset, Scale};

fn main() {
    let data = include_bytes!("fonts/monserat.ttf");

    let font = Font::from_slice(data);
    let glyph = font
        .glyph('@')
        .build(Scale::uniform(100.0), Offset::uniform(7.0));

    let bitmap = glyph.generate_sdf(14);
    let sdf = bitmap.data;
    let width = bitmap.width;
    let height = bitmap.height;

    let mut image = DynamicImage::new_rgb8(width, height);

    for y in 0..height {
        for x in 0..width {
            let pixel = sdf[(y * width + x) as usize];
            image.put_pixel(x, y, Rgba([pixel, pixel, pixel, 255]));
        }
    }

    image.save("examples/test.png").unwrap();
}
