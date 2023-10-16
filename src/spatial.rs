// Coordinate type alias; for ease of use.
pub type Coordinate = (f64, f64);

#[derive(Debug)]
pub struct Region {
    pub bottom_left: Coordinate,
    pub top_right: Coordinate,
}