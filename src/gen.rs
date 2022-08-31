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
    // Each contour with signed distance to the `pixel`.
    let mut sorted_contours = shape
        .iter()
        .map(|contour| contour.get_data(pixel))
        .collect::<Vec<_>>();
    // Sort all contours by their distance to pixel from closest to furthest.
    sorted_contours
        .sort_by(|a, b| a.distance.partial_cmp(&b.distance).unwrap());

    let closest_contour = sorted_contours.first().expect(&format!("This shape contains no contours: {:?}", shape));
    //println!("closest: {:?}", closest_contour);

    // ______________Overlapping contours correction________________

    // // // // FIRST CHECK IF THERE ARE ANY INTERSECTIONS // // // //
    // TODO maybe transfer all below code into another function
    if !shape.has_overlaps() {
        return closest_contour.distance;
    }

    // TODO IMPORTANT then check if shortest distance's contour has any intersections
    // or is surrounded by a contour with the same winding

    // TODO IMPORTANT store all segment intersection points and check if cw contour 
    // is contained in surrounding cw contour and ccw contour is contained in surrounding
    // ccw contour and store the results (checking for duplicates)

    // cw - clockwise
    // ccw - counter clockwise
    // IMPORTANT: clockwise contours have advantage over counter clockwise 
    // contours only when they overlap

    // Next steps are:
    // - check if there are any surrounding contours,
    // - sort all surrounding contours from closest to furthest,
    // - TODO ...

    // Since we sorted all contour from the start, all following contour
    // lists will be sorted.

    // Gets all contours surrounding the current pixel. 
    // This can be easily achieved by testing:
    // - if distance from contour is positive and contour is clockwise;
    // - else if distance from contour is negative and contour is counter clockwise
    let mut surrounding_contours = sorted_contours
        .iter()
        .filter(|contr| {
            (contr.distance.is_sign_positive() && contr.is_cw())
                || (contr.distance.is_sign_negative()
                    && contr.is_ccw())
        })
        .collect::<Vec<_>>();

    // If there are no surrounding contours return the distance to closest one.
    if surrounding_contours.is_empty() {
        return closest_contour.distance;
    }
    //println!("Sorted Vec: \n {:?}", surrounding_contours);

    let closest_surrounding_contour = surrounding_contours.first().unwrap();
    let cw_surrounding_contours = surrounding_contours
        .iter()
        .filter(|dist| dist.contour_winding.is_cw())
        .collect::<Vec<_>>();
    let ccw_surrounding_contours = surrounding_contours
        .iter()
        .filter(|dist| dist.contour_winding.is_ccw())
        .collect::<Vec<_>>();
    let closest_contour_with_positive_dist = sorted_contour_distances
        .iter()
        .find(|dist| dist.distance.sign.is_sign_positive())
        .unwrap();
    //let closest_surrounding_contour_with_negative_dist_index =
    //    sorted_contour_distances
    //        .iter()
    //        .position(|dist| dist.distance.sign.is_sign_negative())
    //        .or(Some(surrounding_contours.len()))
    //        .unwrap();
    // If there is only one clockwise surrounding contour it will be returned.
    let furthest_cw_surrounding_contour = {
        let mut peekable = surrounding_contours.iter().peekable();
        let mut result = None;
        while let Some(item) = peekable.next() {
            if let Some(next) = peekable.peek() {
                if next.contour_winding.is_ccw() {
                    result = Some(item);
                    break;
                }
                continue;
            }
            result = Some(item);
            break;
        }
        result
    }.unwrap();
    let closest_ccw_contour_with_positive_dist = sorted_contour_distances
        .iter()
        .find(|dist| {
            dist.contour_winding.is_ccw() && dist.distance.sign.is_sign_positive()
        });

    let has_cw_surrounding_contours = !cw_surrounding_contours.is_empty();

    // If pixel is not surrounded by any clockwise contours
    // then it should have negative dist.
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

    // Now it is guaranteed that the shortest distance is negative and should be positive or is positive
    // and should be negative.
    // assert!(shortest_dist.sign.is_sign_negative());

    // If the closest surrounding contour is clockwise then the distance is
    // guaranteed to be positive.
    // TODO check if positive with assert
    // TODO test this whole portion with custom shapes
    if closest_surrounding_contour.is_cw() {
        // TODO create a struct for pixels and contourData, add a function to check if contour contains a pixel
        if shortest_dist.sign.is_sign_positive() {
            if closest_winding.is_cw() {
                    if let Some(closest_ccw) = closest_ccw_contour_with_positive_dist {
                        if furthest_cw_surrounding_contour.distance > closest_ccw.distance {
                            return closest_ccw.distance;
                        }
                    }
                    //println!("furthest: {:?}", furthest_cw_surrounding_contour);
                    return furthest_cw_surrounding_contour.distance;
            } else {
                if shape.are_overlapping(closest_contour_id, closest_surrounding_contour.contour_id) {
                    return closest_surrounding_contour.distance;
                } else {
                    return shortest_dist;
                }
            }
        } else {
            if let Some(closest_ccw) = closest_ccw_contour_with_positive_dist {
                if furthest_cw_surrounding_contour.distance > closest_ccw.distance {
                    return closest_ccw.distance;
                }
            }
            return furthest_cw_surrounding_contour.distance;
        }
        // TODO Should be the furthest clockwise contour with no counter clockwise contours in between
        // TODO test if the distance is to furthest cw contour without ccw in between
        //if closest_surrounding_ccw_index > 1 {
        //    let furthest_surrounding_cw = surrounding_contours
        //        .get(closest_surrounding_ccw_index - 1)
        //        .unwrap();
        //    return furthest_surrounding_cw.distance;
        //    //if let Some(second) = furthest_surrounding_cw {
        //    //    if second.contour_winding.is_cw() {
        //    //        return second.distance;
        //    //    }
        //    //}
        //}
        //return shortest_dist;
        //return closest_contour_with_positive_dist.distance;
    } else {
        // Pixel is either surrounded by a counter clockwise contour or is also surrounded
        // by an clockwise contour which intersects counter clockwise one and
        // should have positive distance.
        // TODO add an overlap check
        let closest_ccw = closest_surrounding_contour;
        let closest_intersecting_cw = cw_surrounding_contours
            .iter()
            .filter(|dist| {
                shape.are_overlapping(dist.contour_id, closest_ccw.contour_id)
            })
            .reduce(|accum, item| {
                if accum.distance < item.distance {
                    accum
                } else {
                    item
                }
            });
        if let Some(intersecting) = closest_intersecting_cw {
            return intersecting.distance;
        }
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
