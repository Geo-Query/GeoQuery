use crate::index::MetaData;
use crate::parsing::dted::DT2MetaData;
use crate::parsing::geojson::GeoJSONMetaData;
use crate::parsing::gpkg::GPKGMetaData;
use crate::parsing::kml::KMLMetadata;
use crate::parsing::mbtiles::MBTilesMetaData;
use crate::parsing::shapefile::ShapeFileMetaData;
use geotiff::GeoTiffMetaData;

impl From<KMLMetadata> for MetaData {
    fn from(value: KMLMetadata) -> Self {
        MetaData {
            region: value.region.into(),
            tags: value.tags,
        }
    }
}

impl From<GeoTiffMetaData> for MetaData {
    fn from(value: GeoTiffMetaData) -> Self {
        MetaData {
            region: value.region.into(),
            tags: value.tags,
        }
    }
}

impl From<DT2MetaData> for MetaData {
    fn from(value: DT2MetaData) -> Self {
        MetaData {
            region: value.region.into(),
            tags: value.tags,
        }
    }
}

impl From<GeoJSONMetaData> for MetaData {
    fn from(value: GeoJSONMetaData) -> Self {
        let x = MetaData {
            region: value.region.into(),
            tags: value.tags,
        };
        return x;
    }
}

impl From<MBTilesMetaData> for MetaData {
    fn from(value: MBTilesMetaData) -> Self {
        MetaData {
            region: value.region.into(),
            tags: value.tags,
        }
    }
}

impl From<GPKGMetaData> for MetaData {
    fn from(value: GPKGMetaData) -> Self {
        MetaData {
            region: value.region.into(),
            tags: value.tags,
        }
    }
}

impl From<ShapeFileMetaData> for MetaData {
    fn from(value: ShapeFileMetaData) -> Self {
        MetaData {
            region: value.region,
            tags: value.tags,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parsing::dted::DT2Region;
    use crate::parsing::geojson::GeoJSONRegion;
    use crate::parsing::gpkg::GPKGRegion;
    use crate::parsing::kml::KMLRegion;
    use crate::parsing::mbtiles::MBTilesRegion;
    use crate::spatial::Coordinate;
    use crate::spatial::Region;
    use geotiff::GeoTiffRegion;
    use std::collections::HashMap;

    #[test]
    fn test_kml_metadata_conversion() {
        let kml_region = KMLRegion {
            top_right: (45.0, -90.0), // Corrected to use tuple syntax
            bottom_left: (40.0, -95.0),
        };
        let kml_metadata = KMLMetadata {
            region: kml_region,
            tags: vec![("creator".to_string(), "test".to_string())],
        };
        let meta_data: MetaData = From::from(kml_metadata);
        // Ensure that your assertions are compatible with how you interpret the region for MetaData
        assert_eq!(meta_data.region.top_left, (40.0, -90.0)); // Corrected expected value
        assert_eq!(
            meta_data.tags,
            vec![("creator".to_string(), "test".to_string())]
        );
    }

    #[test]
    fn region_conversion() {
        let top_left: Coordinate = (10.0, 20.0);
        let bottom_right: Coordinate = (15.0, 25.0);
        let region = Region {
            top_left,
            bottom_right,
        };

        assert_eq!(region.top_left, top_left);
        assert_eq!(region.bottom_right, bottom_right);
    }

    // Test for another metadata conversion as an example
    #[test]
    fn geotiff_to_metadata_conversion() {
        let geotiff_region = GeoTiffRegion {
            top_left: (30.0, -120.0),
            bottom_right: (35.0, -115.0),
        };
        let geotiff_metadata = GeoTiffMetaData {
            region: geotiff_region,
            tags: vec![("resolution".to_string(), "high".to_string())],
        };
        let meta_data: MetaData = geotiff_metadata.into();
        assert_eq!(meta_data.region.top_left, (30.0, -120.0));
        assert_eq!(meta_data.region.bottom_right, (35.0, -115.0));
        assert_eq!(
            meta_data.tags,
            vec![("resolution".to_string(), "high".to_string())]
        );
    }

    // Test the conversion from DT2MetaData to MetaData
    #[test]
    fn dt2_to_metadata_conversion() {
        let dt2_region = DT2Region {
            top_left: (50.0, -100.0),
            top_right: (50.0, -95.0),
            bottom_right: (45.0, -95.0),
            bottom_left: (45.0, -100.0),
        };
        let dt2_metadata = DT2MetaData {
            region: dt2_region,
            tags: vec![("source".to_string(), "satellite".to_string())],
        };
        let meta_data: MetaData = dt2_metadata.into();
        // Assuming the conversion uses top_left and bottom_right for the MetaData region
        assert_eq!(meta_data.region.top_left, (50.0, -100.0));
        assert_eq!(meta_data.region.bottom_right, (45.0, -95.0));
        assert_eq!(
            meta_data.tags,
            vec![("source".to_string(), "satellite".to_string())]
        );
    }

    // Test the conversion from GeoJSONMetaData to MetaData
    #[test]
    fn geojson_to_metadata_conversion() {
        let geojson_region = GeoJSONRegion {
            top_right: (60.0, -80.0),
            bottom_left: (55.0, -85.0),
        };
        let geojson_metadata = GeoJSONMetaData {
            region: geojson_region,
            tags: vec![("type".to_string(), "feature".to_string())],
        };
        let meta_data: MetaData = geojson_metadata.into();

        assert_eq!(meta_data.region.top_left, (55.0, -80.0));
        assert_eq!(meta_data.region.bottom_right, (60.0, -85.0));
        assert_eq!(
            meta_data.tags,
            vec![("type".to_string(), "feature".to_string())]
        );
    }

    // Test the conversion from MBTilesMetaData to MetaData
    #[test]
    fn mbtiles_to_metadata_conversion() {
        let mbtiles_region = MBTilesRegion {
            top_left: (70.0, -60.0),
            bottom_right: (65.0, -55.0),
        };
        let mbtiles_metadata = MBTilesMetaData {
            region: mbtiles_region,
            tags: vec![("zoom_level".to_string(), "15".to_string())],
        };
        let meta_data: MetaData = mbtiles_metadata.into();
        assert_eq!(meta_data.region.top_left, (70.0, -60.0));
        assert_eq!(meta_data.region.bottom_right, (65.0, -55.0));
        assert_eq!(
            meta_data.tags,
            vec![("zoom_level".to_string(), "15".to_string())]
        );
    }

    // Test the conversion from GPKGMetaData to MetaData
    #[test]
    fn gpkg_to_metadata_conversion() {
        let gpkg_region = GPKGRegion {
            top_left: (30.0, -120.0),
            bottom_right: (25.0, -115.0),
        };
        let gpkg_metadata = GPKGMetaData {
            region: gpkg_region,
            tags: vec![("table_name".to_string(), "geodata".to_string())],
        };
        let meta_data: MetaData = gpkg_metadata.into();
        assert_eq!(meta_data.region.top_left, (30.0, -120.0));
        assert_eq!(meta_data.region.bottom_right, (25.0, -115.0));
        assert_eq!(
            meta_data.tags,
            vec![("table_name".to_string(), "geodata".to_string())]
        );
    }

    // Test the conversion from ShapeFileMetaData to MetaData
    #[test]
    fn shapefile_to_metadata_conversion() {
        let shapefile_region = Region {
            top_left: (40.0, -110.0),
            bottom_right: (35.0, -105.0),
        };
        let shapefile_metadata = ShapeFileMetaData {
            region: shapefile_region,
            tags: vec![("attribute".to_string(), "value".to_string())],
        };
        let meta_data: MetaData = shapefile_metadata.into();
        assert_eq!(meta_data.region.top_left, (40.0, -110.0));
        assert_eq!(meta_data.region.bottom_right, (35.0, -105.0));
        assert_eq!(
            meta_data.tags,
            vec![("attribute".to_string(), "value".to_string())]
        );
    }
    #[test]
    fn test_metadata_conversion_with_empty_tags() {
        let geojson_region = GeoJSONRegion {
            top_right: (60.0, -80.0),
            bottom_left: (55.0, -85.0),
        };
        let geojson_metadata = GeoJSONMetaData {
            region: geojson_region,
            tags: Vec::new(), // Empty tag list
        };
        let meta_data: MetaData = geojson_metadata.into();
        assert!(meta_data.tags.is_empty());
    }

    // Test handling a large number of tags
    #[test]
    fn test_handling_large_amount_of_tags() {
        let mbtiles_region = MBTilesRegion {
            top_left: (70.0, -60.0),
            bottom_right: (65.0, -55.0),
        };
        let mut tags = Vec::new();
        for i in 0..1000 {
            // Generate 1000 tags
            tags.push((format!("key{}", i), format!("value{}", i)));
        }
        let mbtiles_metadata = MBTilesMetaData {
            region: mbtiles_region,
            tags,
        };
        let meta_data: MetaData = mbtiles_metadata.into();
        assert_eq!(meta_data.tags.len(), 1000);
    }

    #[test]
    fn test_empty_tags_conversion() {
        let kml_region = KMLRegion {
            top_right: (45.0, -90.0),
            bottom_left: (40.0, -95.0),
        };
        let kml_metadata = KMLMetadata {
            region: kml_region,
            tags: Vec::new(), // Empty tag vector
        };
        let meta_data: MetaData = From::from(kml_metadata);
        assert!(meta_data.tags.is_empty()); // Confirm that the converted tags vector is empty
    }

    // Test conversion with multiple tags
    #[test]
    fn test_multiple_tags_conversion() {
        let geojson_region = GeoJSONRegion {
            top_right: (60.0, -80.0),
            bottom_left: (55.0, -85.0),
        };
        let geojson_metadata = GeoJSONMetaData {
            region: geojson_region,
            tags: vec![
                ("type".to_string(), "feature".to_string()),
                ("source".to_string(), "user".to_string()),
            ],
        };
        let meta_data: MetaData = From::from(geojson_metadata);

        assert_eq!(meta_data.tags.len(), 2);
        assert!(meta_data
            .tags
            .contains(&("type".to_string(), "feature".to_string())));
        assert!(meta_data
            .tags
            .contains(&("source".to_string(), "user".to_string())));
    }

    // Test conversion with region boundary values
    #[test]
    fn test_region_boundary_values_conversion() {
        let gpkg_region = GPKGRegion {
            top_left: (90.0, -180.0),
            bottom_right: (-90.0, 180.0),
        };
        let gpkg_metadata = GPKGMetaData {
            region: gpkg_region,
            tags: vec![("coverage".to_string(), "global".to_string())],
        };
        let meta_data: MetaData = From::from(gpkg_metadata);

        assert_eq!(meta_data.region.top_left, (90.0, -180.0));
        assert_eq!(meta_data.region.bottom_right, (-90.0, 180.0));
        assert_eq!(
            meta_data.tags,
            vec![("coverage".to_string(), "global".to_string())]
        );
    }
}
