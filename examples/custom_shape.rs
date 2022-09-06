use image::{DynamicImage, GenericImage, Rgba};
use msdfont::{PathBuilder, GlyphOutline, Scale};

fn main() {
    let mut builder = PathBuilder::new_with_scale(Scale::new(100.0));
    builder.open_at(10.0, 10.0);
    builder.line_to(5.0, 5.0);
    builder.line_to(10.0, 5.0);
    builder.line_to(10.0, 10.0);
    builder.close();
    let shape = builder.build_shape();

    let glyph = GlyphOutline::from_shape(shape);

    let width = glyph.width() as u32;
    let height = glyph.height() as u32;
    let offset = 5;

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
