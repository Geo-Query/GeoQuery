use std::fmt::{Debug};
use serde::{Deserialize, Serialize};
use geotiff::GeoTiffRegion;
use crate::io::QueryRegion;
use crate::parsing::dt2::DT2Region;
use crate::parsing::geojson::GeoJSONRegion;
use crate::parsing::kml::KMLRegion;

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

#[cfg(test)]
mod tests {
    use super::*;

    // Test conversion from GeoTiffRegion to Region
    #[test]
    fn test_convert_from_geotiff_region() {
        let geo_tiff_region = GeoTiffRegion {
            top_left: (1.0, 2.0),
            bottom_right: (3.0, 4.0),
        };
        let region: Region = geo_tiff_region.into();
        assert_eq!(region.top_left, (1.0, 2.0));
        assert_eq!(region.bottom_right, (3.0, 4.0));
    }

    // Test conversion from KMLRegion to Region
    #[test]
    fn test_convert_from_kml_region() {
        let kml_region = KMLRegion {
            bottom_left: (1.0, 2.0),
            top_right: (3.0, 4.0),
        };
        let region: Region = kml_region.into();
        assert_eq!(region.top_left, (1.0, 4.0));
        assert_eq!(region.bottom_right, (3.0, 2.0));
    }

    // Test conversion from GeoJSONRegion to Region
    #[test]
    fn test_convert_from_geojson_region() {
        let geojson_region = GeoJSONRegion {
            bottom_left: (1.0, 2.0),
            top_right: (3.0, 4.0),
        };
        let region: Region = geojson_region.into();
        assert_eq!(region.top_left, (1.0, 4.0));
        assert_eq!(region.bottom_right, (3.0, 2.0));
    }

    // Test conversion from DT2Region to Region
    #[test]
    fn test_convert_from_dt2_region() {
        let dt2_region = DT2Region {
            top_left: (1.0, 2.0),
            top_right: (3.0, 2.0), 
            bottom_left: (1.0, 4.0), 
            bottom_right: (3.0, 4.0),
        };
        let region: Region = dt2_region.into();
        assert_eq!(region.top_left, (1.0, 2.0));
        assert_eq!(region.bottom_right, (3.0, 4.0));
    }

}
