use crate::{
    font::GlyphOutline, math::SignedDistance, shape::Shape, vector::Vector2,
};

pub struct Bitmap {
    data: Vec<u8>,
    width: u32,
    height: u32,
}

impl Bitmap {
    pub fn data(self) -> Vec<u8> {
        self.data
    }
}

pub fn generate_sdf(outline: &GlyphOutline, pxrange: usize) -> Bitmap {
    let shape = &outline.shape;
    let mut data = Vec::new();
    let width = outline.bbox.width();
    let height = outline.bbox.height();
    println!("width: {}, height: {}", width, height);
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
        width: width as u32,
        height: height as u32,
    }
}

pub fn pixel_distance(shape: &Shape, pixel: Vector2) -> SignedDistance {
    // Each contour with signed distance to the `pixel`.
    let mut sorted_contours = shape
        .iter()
        .map(|contour| contour.get_data(pixel))
        .collect::<Vec<_>>();
    // Sort all contours by their distance to pixel from closest to furthest.
    sorted_contours.sort_by(|a, b| a.distance.partial_cmp(&b.distance).unwrap());

    let closest_contour = sorted_contours
        .first()
        .expect(&format!("This shape contains no contours: {:?}", shape));
    return closest_contour.distance;
    //println!("closest: {:?}", closest_contour);
    /*
    // ______________Overlapping contours correction________________

    // // // // FIRST CHECK IF THERE ARE ANY INTERSECTIONS // // // //
    // TODO maybe transfer all below code into another function
    if !shape.overlaps_exist() {
        return closest_contour.distance;
    }

    // TODO IMPORTANT then check if shortest distance's contour has any intersections
    // or is surrounded by a contour with the same winding

    // if shape.has_intersections(closest_contour.id) {
    //
    // }

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
    let surrounding_contours = sorted_contours
        .iter()
        .filter(|contour| contour.is_surrounding())
        .collect::<Vec<_>>();

    // If there are no surrounding contours return the distance to closest one.
    if surrounding_contours.is_empty() {
        return closest_contour.distance;
    }
    //println!("Sorted Vec: \n {:?}", surrounding_contours);

    let closest_surrounding_contour = surrounding_contours.first().unwrap();
    let closest_surrounding_contour_index = sorted_contours.iter().position(|contour| {
        contour.is_surrounding()
    }).unwrap();
    // Represents the furthest contour that surrounds the same winding closest contour
    // without opposite winding contours is between.
    // let furthest_same_winding_contour = ;
    // Represents all contours by distance from the closest one to the closest surrounding contour.
    let contours_to_closest_surrounding = &(sorted_contours[0..closest_surrounding_contour_index]);

    let cw_surrounding_contours = surrounding_contours
        .iter()
        .filter(|dist| dist.is_cw())
        .collect::<Vec<_>>();
    let ccw_surrounding_contours = surrounding_contours
        .iter()
        .filter(|dist| dist.is_ccw())
        .collect::<Vec<_>>();
    let closest_contour_with_positive_dist = sorted_contours
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
                if next.is_ccw() {
                    result = Some(item);
                    break;
                }
                continue;
            }
            result = Some(item);
            break;
        }
        result
    }
    .unwrap();
    let closest_ccw_contour_with_positive_dist = sorted_contours
        .iter()
        .find(|dist| dist.is_ccw() && dist.distance.is_sign_positive());

    // If pixel is not surrounded by any clockwise contours
    // then it should have negative dist.
    if cw_surrounding_contours.is_empty() {
        return closest_surrounding_contour.distance;
    }

    // List of all non surrounding ccw contours.
    let non_surrounding_ccw_contours = sorted_contours
        .iter()
        .filter(|contour| contour.is_ccw() && !contour.is_surrounding())
        .collect::<Vec<_>>();

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
        if closest_contour.distance.is_sign_positive() {
            // Pixel is closest to surrounding clockwise contour or to non surrounding
            // counter clockwise contour.
            if closest_contour.is_cw() {
                // Pixel is guaranteed to be closest to the closest surrounding contour.
                if let Some(closest_ccw) = closest_ccw_contour_with_positive_dist {
                    if furthest_cw_surrounding_contour.distance
                        > closest_ccw.distance
                    {
                        return closest_ccw.distance;
                    }
                }
                //println!("furthest: {:?}", furthest_cw_surrounding_contour);
                return furthest_cw_surrounding_contour.distance;
            } else {
                // Pixel is closest to a non surrounding counterclockwise contour
                // (that is surrounded by or that intersects the surrounding clockwise contour).
                // TODO should return all intersection points.
                let intersections = shape.get_intersections(
                    closest_contour.id,
                    closest_surrounding_contour.id,
                );
                if intersections {
                    return closest_surrounding_contour.distance;
                } else {
                    // No intersections meaning ccw contour is just surrounded by
                    // the surrounding cw contour.
                    return closest_contour.distance;
                }
            }
        } else {
            // Distance sign is negative.
            // Pixel is closest to a non surrounding clockwise contour
            // (that is surrounded by or that intersects the surrounding clockwise contour).
            if let Some(closest_ccw) = closest_ccw_contour_with_positive_dist {
                if furthest_cw_surrounding_contour.distance > closest_ccw.distance {
                    return closest_ccw.distance;
                }
            }
            return furthest_cw_surrounding_contour.distance;
        }
    } else {
        // Pixel is either surrounded by a counter clockwise contour or is also surrounded
        // by an clockwise contour which intersects counter clockwise one and
        // should have positive distance.
        // TODO add an overlap check
        let closest_ccw = closest_surrounding_contour;
        let closest_intersecting_cw = cw_surrounding_contours
            .iter()
            .filter(|dist| shape.get_intersections(dist.id, closest_ccw.id))
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
        return closest_contour.distance;
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
    //}*/
}
