use rusttype::{point, Font, OutlineBuilder, Scale};

struct Builder {}

impl OutlineBuilder for Builder {
    fn move_to(&mut self, x: f32, y: f32) {
        println!("pocinjem na: {} {}", x, y);
    }

    fn line_to(&mut self, x: f32, y: f32) {
        println!("crta do: {} {}", x, y);
    }

    fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) {
        println!(
            "kvadraticna parabola: x1: {}, y1: {}, x: {}, y: {}",
            x1, y1, x, y
        );
    }

    fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
        println!(
            "kubna parabola: x1: {}, y1: {}, x2: {}, y2: {} x: {}, y: {}",
            x1, y1, x2, y2, x, y
        );
    }

    fn close(&mut self) {
        println!("_________kraj________");
    }
}

fn main() {
    let data = include_bytes!("monserat.ttf");
    let font = Font::try_from_bytes(data).unwrap();

    let scale = Scale::uniform(32.0);

    let char = 'P';

    let glyph = font.glyph(char).scaled(scale);

    let mut builder = Builder {};

    glyph.build_outline(&mut builder);
}
