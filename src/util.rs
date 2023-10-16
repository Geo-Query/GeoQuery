use crate::spatial::Coordinate;

pub fn draw_boundaries(coordinates: Vec<Coordinate>) -> (Coordinate, Coordinate) {
    let mut min_x: f64 = coordinates[0].0;
    let mut min_y: f64 = coordinates[0].1;
    let mut max_x: f64 = coordinates[0].0;
    let mut max_y: f64 = coordinates[0].1;
    for coordinate in coordinates {
        if coordinate.0 > max_x {
            max_x = coordinate.0;
        }
        if coordinate.0 < min_x {
            min_x = coordinate.0;
        }
        if coordinate.1 > max_y {
            max_y = coordinate.1;
        }
        if coordinate.1 < min_y {
            min_y = coordinate.1;
        }
    }
    return ((min_x, min_y), (max_x, max_y));
}