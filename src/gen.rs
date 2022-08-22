use crate::{math::SignedDistance, shape::Shape, vector::Vector2};

pub struct Bitmap {
    data: Vec<u8>,
    width: u32,
    height: u32,
}

impl Bitmap {}

pub fn generate_sdf(shape: &Shape, pxrange: usize) -> Bitmap {
    let mut data = Vec::new();
    let width = 1;
    let height = 1;
    for y in 0..height {
        for x in 0..width {
            let pixel = Vector2::new(x as f32 + 0.5, y as f32 + 0.5);

            let distance = pixel_distance(shape, pixel);
            let signed_distance = distance.sign * distance.real_dist;

            // Used for normal SDF
            let normalized = (signed_distance / pxrange as f32) + 0.5;
            // Used for pseudo-SDF
            //let pseudo = ((distance.sign * distance.extended_dist
            //    / pxrange)
            //    + 0.5)
            //    .clamp(0.0, 1.0);

            // When f32 is being converted to u8 it is automatically
            // clamped in range [0, 255].
            let sdf = (normalized * 255.0) as u8;

            data.push(sdf);
        }
    }

    Bitmap {
        data,
        width,
        height,
    }
}

pub fn pixel_distance(shape: &Shape, pixel: Vector2) -> SignedDistance {
    // Distances from pixel to each contour with contours winding.
    let contour_distances = shape
        .iter()
        .map(|contour| contour.get_distance_from(pixel))
        .collect::<Vec<_>>();

    let closest_contour_dist = contour_distances
        .iter()
        .reduce(|accum_dist, item_dist| {
            if accum_dist.distance < item_dist.distance {
                accum_dist
            } else {
                item_dist
            }
        })
        .unwrap();

    let shortest_dist = closest_contour_dist.distance;
    println!("closest: {:?}", shortest_dist);
    let closest_winding = closest_contour_dist.contour_winding;

    // ______________Overlapping contours correction________________

    // 1) Check if correction is NOT needed:
    // a) Check if should be filled by default (pixel's closest distance is positive):
    if shortest_dist.sign.is_sign_positive() {
        return shortest_dist;
    }
    // b) Check if pixel is closest to the surrounding counter clockwise contour:
    //if shortest_dist.sign.is_sign_negative() && closest_winding.is_ccw() {
    //    return shortest_dist;
    //}

    // After the first two checks we are sure that the pixel is not placed inside
    // and closest to the surrounding contour (if there are any surrounding it).
    // Next steps are:
    // - check if there are any surrounding contours,
    // - sort all surrounding contours from closest to furthest,
    // - if { the closest contour is clockwise} return distance to it,
    // - else if { the closest contour is counter clockwise } return shortest distance.

    // TODO if pixel in closest to surrounding clockwise contour check if there is 
    // second close clockwise contour surrounding it.

    // Get all contours surrounding current pixel.
    let mut surrounding_contours = contour_distances
        .iter()
        .filter(|dist| {
            (dist.distance.sign.is_sign_positive() && dist.contour_winding.is_cw())
                || (dist.distance.sign.is_sign_negative()
                    && dist.contour_winding.is_ccw())
        })
        .collect::<Vec<_>>();

    // If there are no surrounding contours return the closest one.
    if surrounding_contours.is_empty() {
        return shortest_dist;
    }

    // Sort all contours by their distance to pixel from closest to furthest.
    surrounding_contours
        .sort_by(|a, b| a.distance.partial_cmp(&b.distance).unwrap());
    println!("Sorted Vec: \n {:?}", surrounding_contours);

    let closest_surrounding_contour = surrounding_contours.first().unwrap();

    if closest_surrounding_contour.contour_winding.is_cw() {
        return closest_surrounding_contour.distance;
    } else {
        let closest_cw_surrounding_intersecting_contour = surrounding_contours
        .iter()
        .filter(|dist| dist.contour_winding.is_cw())
        .reduce(|accum, item| {
            if accum.distance < item.distance {
                accum
            } else {
                item
            }
        })
        .unwrap();
        return shortest_dist;
    }

    //let (mut shortest_dist, mut winding) =
    //    (distances.first().unwrap(), windings.first().unwrap());
    //for i in 1..distances.len() {
    //    let dist = distances.get(i).unwrap();
    //    let w = windings.get(i).unwrap();
    //    if dist.sign != shortest_dist.sign {
    //        if winding == w {
    //            if dist.real_dist.abs() < shortest_dist.real_dist.abs() {
    //                shortest_dist = dist;
    //                winding = w;
    //                continue;
    //            }
    //        }
    //    }
    //    if dist < shortest_dist {
    //        shortest_dist = dist;
    //        winding = w;
    //    }
    //}
}
