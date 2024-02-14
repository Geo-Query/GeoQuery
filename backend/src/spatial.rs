use std::fmt::{Debug};
use serde::{Deserialize, Serialize};
use geotiff::GeoTiffRegion;
use crate::io::QueryRegion;
use crate::parsing::dt2::DT2Region;
use crate::parsing::geojson::GeoJSONRegion;
use crate::parsing::kml::KMLRegion;
use crate::parsing::mbtiles::MBTilesRegion;
use crate::parsing::gpkg::GPKGRegion;

// Coordinate type alias; for ease of use.
pub type Coordinate = (f64, f64);


// Region unit type. All region types should implement From.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Region {
    pub top_left: Coordinate,
    pub bottom_right: Coordinate
}

// Conversions of myriad format specific region types into std unit type.
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

impl From<QueryRegion> for Region {
    fn from(value: QueryRegion) -> Self {
        Region {
            top_left: (value.top_left_long, value.top_left_lat),
            bottom_right: (value.bottom_right_long, value.bottom_right_lat)
        }
    }
}

impl From<MBTilesRegion> for Region {
    fn from(t: MBTilesRegion) -> Region {
        Region {
            top_left: t.top_left,
            bottom_right: t.bottom_right
        }
    }
}

impl From<GPKGRegion> for Region {
    fn from(t: GPKGRegion) -> Region {
        Region {
            top_left: t.top_left,
            bottom_right: t.bottom_right
        }
    }
}
