use std::fmt::{Debug, Formatter};
use geotiff::GeoTiffRegion;

// Coordinate type alias; for ease of use.
pub type Coordinate = (f64, f64);


pub trait Region: Debug {
    fn bottom_left(&self) -> Coordinate;
    fn bottom_right(&self) -> Coordinate;

    fn top_left(&self) -> Coordinate;

    fn top_right(&self) -> Coordinate;
}



impl Region for GeoTiffRegion {
    fn bottom_left(&self) -> Coordinate {
        self.bottom_left
    }

    fn bottom_right(&self) -> Coordinate {
        (self.top_right.0, self.bottom_left.1)
    }

    fn top_left(&self) -> Coordinate {
        (self.bottom_left.0, self.top_right.1)
    }

    fn top_right(&self) -> Coordinate {
        self.top_right
    }
}