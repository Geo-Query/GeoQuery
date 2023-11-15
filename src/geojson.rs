use std::fs::File;
use std::io::BufReader;
use crate::spatial::{Coordinate, Region};

#[derive(Debug)]
pub struct GeoJSONRegion {
    top_left: Coordinate,
    bottom_right: Coordinate
}

impl Region for GeoJSONRegion {
    fn bottom_left(&self) -> Coordinate {
        todo!()
    }

    fn bottom_right(&self) -> Coordinate {
        self.bottom_right
    }

    fn top_left(&self) -> Coordinate {
        self.top_left
    }

    fn top_right(&self) -> Coordinate {
        todo!()
    }
}

pub fn parse_geojson(reader: &mut BufReader<File>) {
    todo!()
}