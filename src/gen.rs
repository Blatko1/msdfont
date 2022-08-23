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
        .map(|contour| contour.get_distance(pixel))
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

    // // // // FIRST CHECK IF THERE ARE ANY INTERSECTIONS // // // //
    // if !intersecting() {
    //     return shortest_dist;
    // }

    // TODO IMPORTANT then check if shortest distance's contour has any intersections

    // cw - clockwise
    // ccw - counter clockwise
    // IMPORTANT: clockwise contours have advantage over
    // counter clockwise contours if they overlap

    // 1) Check if correction is NOT needed:
    // a) Check if should be filled by default (pixel's closest distance is positive):
    //if shortest_dist.sign.is_sign_positive() {
    //    return shortest_dist;
    //}
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

    // Get all contours surrounding current pixel. This can be easily achieved
    // by checking:
    // - if distance from contour is positive and contour is clockwise then the
    // contour is surrounding the pixel;
    // - else if distance from contour is negative and contour is counter clockwise then
    // the contour is surrounding the pixel
    let mut surrounding_contours = contour_distances
        .iter()
        .filter(|dist| {
            (dist.distance.sign.is_sign_positive() && dist.contour_winding.is_cw())
                || (dist.distance.sign.is_sign_negative()
                    && dist.contour_winding.is_ccw())
        })
        .collect::<Vec<_>>();

    // If there are no surrounding contours return the distance to closest one.
    if surrounding_contours.is_empty() {
        return shortest_dist;
    }

    // Sort all contours by their distance to pixel from closest to furthest.
    surrounding_contours
        .sort_by(|a, b| a.distance.partial_cmp(&b.distance).unwrap());
    println!("Sorted Vec: \n {:?}", surrounding_contours);

    let closest_surrounding_contour = surrounding_contours.first().unwrap();
    let cw_surrounding_contours = surrounding_contours
        .iter()
        .filter(|dist| dist.contour_winding.is_cw())
        .collect::<Vec<_>>();
    let ccw_surrounding_contours = surrounding_contours
        .iter()
        .filter(|dist| dist.contour_winding.is_ccw())
        .collect::<Vec<_>>();

    let has_cw_surrounding_contours = !cw_surrounding_contours.is_empty();
    // If pixel is not surrounded by any clockwise contours then it should be negative.
    if cw_surrounding_contours.is_empty() {
        return closest_surrounding_contour.distance;
    }

    // TODO FIX DOC
    // If pixel's distance is positive and  return the shortest distance, unless
    // there is a second closest clockwise contour surrounding it in which case
    // it's distance should be returned.
    // This is important because if two clockwise contours overlap (letter "Đ")
    // some pixels would return the distance to the edge inside the other contour
    // which give result where contour outlines are visible inside the overlap.

    // TODO test if not needed
    //if shortest_dist.sign.is_sign_positive() {
    //    // Needed because if two ccw contours overlap.
    //    // If there is no clockwise surrounding contour, maybe it should
    //    if closest_contour_dist.contour_winding.is_cw() {
    //        // TODO Should be the furthest clockwise contour with no counter clockwise contours in between
    //        if let Some(second) = surrounding_contours.get(1) {
    //            if second.contour_winding.is_cw() {
    //                return second.distance;
    //            }
    //        }
    //        return shortest_dist;
    //    }
    //}

    // Now it is guaranteed that the shortest distance is negative and should be positive or is positive
    // and should be negative.
    // assert!(shortest_dist.sign.is_sign_negative());

    // If the closest surrounding contour is clockwise then the distance 
    // should always be positive. TODO check if positive with assert
    // If it is surrounded by at least one then proceed with the checks.
    if closest_surrounding_contour.contour_winding.is_cw() {
        if shortest_dist.sign.is_sign_positive() {
            // TODO Should be the furthest clockwise contour with no counter clockwise contours in between
            if let Some(second) = surrounding_contours.get(1) {
                if second.contour_winding.is_cw() {
                    return second.distance;
                }
            }
            return shortest_dist;
        }
        return closest_surrounding_contour.distance;
    } else {
        // Pixel is either surrounded by a counter clockwise contour or is also surrounded
        // by an clockwise contour which intersects counter clockwise one.
        // TODO add an overlap check
        let closest_ccw = closest_surrounding_contour;
        let closest_intersecting_cw = cw_surrounding_contours
            .iter()
            .reduce(|accum, item| {
                if accum.distance < item.distance {
                    accum
                } else {
                    item
                }
            });
        //if let Some(intersecting) = closest_intersecting_cw {
        //    return intersecting.distance;
        //}
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
