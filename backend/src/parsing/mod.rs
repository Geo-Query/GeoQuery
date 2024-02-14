use std::sync::Arc;
use std::error::Error;
use tracing::{span, Level};
use std::fs::File;
use std::io::BufReader;
use geotiff::{parse_tiff};
use crate::{MapType};
use crate::index::Node;
use crate::parsing::dted::parse_dted;
use crate::parsing::geojson::parse_geojson;
use crate::parsing::kml::parse_kml;
pub mod dted;
pub mod geojson;
pub mod kml;
pub mod conversions;
pub mod error;

pub fn parse(map: Arc<MapType>) -> Result<Option<Node>, Box<dyn Error>> {
    let span = span!(Level::INFO, "Parsing");
    let _guard = span.enter();
    match map.as_ref() {
        MapType::GeoTIFF(tiff) => Ok(Some(Node {
            metadata: parse_tiff(&mut BufReader::new(File::open(&tiff.tiff)?))?.into(),
            map
        })),
        MapType::DTED(dted) => Ok(Some(Node {
            metadata: parse_dted(&mut BufReader::new(File::open(&dted.path)?))?.into(),
            map
        })),
        MapType::KML(kml) => Ok(Some(Node {
            metadata: parse_kml(&mut BufReader::new(File::open(&kml.path)?))?.into(),
            map
        })),
        MapType::GEOJSON(geojson) => Ok(Some(Node {
            metadata: parse_geojson(&mut BufReader::new(File::open(&geojson.path)?))?.into(),
            map
        })),
    }
}

