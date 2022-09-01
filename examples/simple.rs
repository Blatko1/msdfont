use lyon_path::Path;
use lyon_path::math::{point};
use lyon_path::builder::*;

fn main() {
    let data = include_bytes!("fonts/monserat.ttf");

    let mut builder = Path::builder();

    // Build a simple path.
    let mut builder = Path::builder();
    builder.begin(point(0.0, 0.0));
    builder.line_to(point(0.0, 2.0));
    builder.line_to(point(2.0, 2.0));
    builder.line_to(point(0.9, 1.0));
    builder.line_to(point(2.0, 0.0));
    builder.close();

    // Generate the actual path object.
    let path = builder.build();

    for event in &path {
        println!("{:?}", event);
    }
}
