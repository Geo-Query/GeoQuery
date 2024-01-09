use std::fmt::{Debug};
use serde::{Deserialize, Serialize};
use geotiff::GeoTiffRegion;
use crate::parsing::dt2::DT2Region;
use crate::parsing::geojson::GeoJSONRegion;
use crate::parsing::kml::KMLRegion;

// Coordinate type alias; for ease of use.
pub type Coordinate = (f64, f64);


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Region {
    pub top_left: Coordinate,
    pub bottom_right: Coordinate
}

impl Region {
    pub fn bottom_left(&self) -> Coordinate {
        (self.top_left.0, self.bottom_right.1)
    }

    pub fn bottom_right(&self) -> Coordinate {
        self.bottom_right
    }

    pub fn top_left(&self) -> Coordinate {
        self.top_left
    }

    pub fn top_right(&self) -> Coordinate {
        (self.bottom_right.0, self.top_left.1)
    }
}

impl From<GeoTiffRegion> for Region {
    fn from(t: GeoTiffRegion) -> Region {
        Region {
            top_left: t.top_left,
            bottom_right: t.bottom_right
        }
    }
}

impl From<KMLRegion> for Region {
    fn from(t: KMLRegion) -> Region {
        Region {
            top_left: (t.bottom_left.0, t.top_right.1),
            bottom_right: (t.top_right.0, t.bottom_left.1)
        }
    }
}

impl From<GeoJSONRegion> for Region {
    fn from(t: GeoJSONRegion) -> Region {
        Region {
            top_left: (t.bottom_left.0, t.top_right.1),
            bottom_right: (t.top_right.0, t.bottom_left.1)
        }
    }
}

impl From<DT2Region> for Region {
    fn from(t: DT2Region) -> Region {
        Region {
            top_left: t.top_left,
            bottom_right: t.bottom_right
        }
    }
}