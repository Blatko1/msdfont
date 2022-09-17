use crate::{
    font::GlyphOutline, math::Distance, shape::Shape, vector::Vector2,
};

pub struct Bitmap {
    pub data: Vec<u8>,
    pub width: u32,
    pub height: u32,
}

impl Bitmap {
    pub fn data(self) -> Vec<u8> {
        self.data
    }
}

pub fn gen_sdf(outline: &GlyphOutline, range: usize) -> Bitmap {
    let shape = &outline.shape;
    let width = outline.width() + outline.offset.x as i32 * 2;
    let height = outline.height() + outline.offset.y as i32 * 2;
    let mut data = Vec::new();
    println!("width: {}, height: {}", width, height);
    for y in 0..height {
        for x in 0..width {
            let pixel = Vector2::new(x as f32 + 0.5, y as f32 + 0.5);

            let signed_distance =
                shortest_distance(&shape, pixel).real_signed();

            let normalized = (signed_distance / range as f32) + 0.5;

            // When f32 is being converted to u8 it is automatically
            // clamped in range [0, 255].
            let sdf = (normalized * 255.0) as u8;

            data.push(sdf);
        }
    }

    Bitmap {
        data,
        width: width as u32,
        height: height as u32,
    }
}

pub fn gen_pseudo_sdf(outline: &GlyphOutline, range: usize) -> Bitmap {
    let shape = &outline.shape;
    let width = outline.width();
    let height = outline.height();
    let mut data = Vec::new();
    println!("width: {}, height: {}", width, height);
    for y in 0..height as usize {
        for x in 0..width as usize {
            let pixel = Vector2::new(x as f32 + 0.5, y as f32 + 0.5);

            let signed_distance =
                shortest_distance(&shape, pixel).real_signed();

            let normalized = (signed_distance / range as f32) + 0.5;

            // When f32 is being converted to u8 it is automatically
            // clamped in range [0, 255].
            let pseudo = (normalized * 255.0) as u8;

            data.push(pseudo);
        }
    }

    Bitmap {
        data,
        width: width as u32,
        height: height as u32,
    }
}

/// Returns [`Distance`]
fn shortest_distance(shape: &Shape, pixel: Vector2<f32>) -> Distance {
    shape
        .contours
        .iter()
        .map(|contour| contour.distance(pixel))
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .expect("Error: There are no distances??")
}
