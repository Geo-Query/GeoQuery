use crate::io::QueryRegion;
use crate::parsing::dted::DT2Region;
use crate::parsing::geojson::GeoJSONRegion;
use crate::parsing::gpkg::GPKGRegion;
use crate::parsing::kml::KMLRegion;
use crate::parsing::mbtiles::MBTilesRegion;
use geotiff::GeoTiffRegion;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

// Coordinate type alias; for ease of use.
pub type Coordinate = (f64, f64);

// Region unit type. All region types should implement From.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Region {
    pub top_left: Coordinate,
    pub bottom_right: Coordinate,
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
            bottom_right: t.bottom_right,
        }
    }
}

impl From<KMLRegion> for Region {
    fn from(t: KMLRegion) -> Region {
        Region {
            top_left: (t.bottom_left.0, t.top_right.1),
            bottom_right: (t.top_right.0, t.bottom_left.1),
        }
    }
}

impl From<GeoJSONRegion> for Region {
    fn from(t: GeoJSONRegion) -> Region {
        Region {
            top_left: (t.bottom_left.0, t.top_right.1),
            bottom_right: (t.top_right.0, t.bottom_left.1),
        }
    }
}

impl From<DT2Region> for Region {
    fn from(t: DT2Region) -> Region {
        Region {
            top_left: t.top_left,
            bottom_right: t.bottom_right,
        }
    }
}

impl From<QueryRegion> for Region {
    fn from(value: QueryRegion) -> Self {
        Region {
            top_left: (value.top_left_long, value.top_left_lat),
            bottom_right: (value.bottom_right_long, value.bottom_right_lat),
        }
    }
}

impl From<MBTilesRegion> for Region {
    fn from(t: MBTilesRegion) -> Region {
        Region {
            top_left: t.top_left,
            bottom_right: t.bottom_right,
        }
    }
}

impl From<GPKGRegion> for Region {
    fn from(t: GPKGRegion) -> Region {
        Region {
            top_left: t.top_left,
            bottom_right: t.bottom_right,
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
    #[test]
    fn test_convert_from_mbtiles_region() {
        let mbtiles_region = MBTilesRegion {
            top_left: (1.0, 2.0),
            bottom_right: (3.0, 4.0),
        };
        let region: Region = mbtiles_region.into();
        assert_eq!(region.top_left, (1.0, 2.0));
        assert_eq!(region.bottom_right, (3.0, 4.0));
    }
    #[test]
    fn test_convert_from_gpkg_region() {
        let gpkg_region = GPKGRegion {
            top_left: (0.5, 3.5),
            bottom_right: (2.5, 1.5),
        };
        let region: Region = gpkg_region.into();
        assert_eq!(
            region.top_left,
            (0.5, 3.5),
            "GPKGRegion to Region conversion failed for top_left"
        );
        assert_eq!(
            region.bottom_right,
            (2.5, 1.5),
            "GPKGRegion to Region conversion failed for bottom_right"
        );
    }

    #[test]
    fn test_region_methods() {
        let region = Region {
            top_left: (1.0, 4.0),
            bottom_right: (3.0, 2.0),
        };
        assert_eq!(
            region.bottom_left(),
            (1.0, 2.0),
            "bottom_left method failed"
        );
        assert_eq!(region.top_right(), (3.0, 4.0), "top_right method failed");
    }
}
