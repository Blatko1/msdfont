use image::{DynamicImage, GenericImage, Rgba};
use msdfont::{PathBuilder, GlyphOutline, BBox, Vector2};

fn main() {
    let mut builder = PathBuilder::new();
    builder.open_at(-5.0, 5.0);
    builder.line_to(10.0, 5.0);
    builder.line_to(10.0, 0.0);
    builder.line_to(-5.0, 5.0);
    builder.close();
    builder.open_at(0.0, 0.0);
    builder.quad_to(1.0, 9.0, 9.0, 9.0);
    builder.close();
    // TODO resolve this case!
    let shape = builder.build_shape();

    let bbox = BBox::new(Vector2::new(-5, 10), Vector2::new(10, 0));

    let glyph = GlyphOutline::from_shape(shape, bbox);

    let width = glyph.width() as u32;
    let height = glyph.height() as u32;

    let sdf = glyph.generate_sdf(6).data();

    let mut image = DynamicImage::new_rgb8(width, height);

    for y in 0..height {
        for x in 0..width {
            let pixel = sdf[((height - y - 1) * width + x) as usize];
            image.put_pixel(x, y, Rgba([pixel, pixel, pixel, 255]));
        }
    }

    image.save("examples/test.png").unwrap();
}
