use std::fmt::{Debug};
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
        (self.top_left.0, self.bottom_right.1)
    }

    fn bottom_right(&self) -> Coordinate {
        self.bottom_right
    }

    fn top_left(&self) -> Coordinate {
        self.top_left
    }

    fn top_right(&self) -> Coordinate {
        (self.bottom_right.0, self.top_left.1)
    }
}