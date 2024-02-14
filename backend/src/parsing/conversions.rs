use geotiff::GeoTiffMetaData;
use crate::index::MetaData;
use crate::parsing::dted::DT2MetaData;
use crate::parsing::geojson::GeoJSONMetaData;
use crate::parsing::kml::KMLMetadata;
use crate::parsing::shapefile::ShapeFileMetaData;

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
            tags: value.tags
        }
    }
}

impl From<DT2MetaData> for MetaData {
    fn from(value: DT2MetaData) -> Self {
        MetaData {
            region: value.region.into(),
            tags: value.tags
        }
    }
}

impl From<GeoJSONMetaData> for MetaData {
    fn from(value: GeoJSONMetaData) -> Self {
        let x = MetaData {
            region: value.region.into(),
            tags: value.tags
        };
        return x;
    }
}

impl From<ShapeFileMetaData> for MetaData {
    fn from(value: ShapeFileMetaData) -> Self {
        MetaData {
            region: value.region,
            tags: value.tags
        }
    }
}