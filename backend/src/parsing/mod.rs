use std::sync::Arc;
use std::error::Error;
use tracing::{event, span, Level};
use std::fs::File;
use std::io::BufReader;
use std::ffi::OsStr;
use geotiff::{GeoTiffMetaData, parse_tiff};
use crate::FileMeta;
use crate::index::Node;
use crate::parsing::dt2::parse_dt2;
use crate::parsing::error::ParseErrorKind;
use crate::parsing::geojson::parse_geojson;
use crate::parsing::kml::parse_kml;
pub mod dt2;
pub mod geojson;
pub mod kml;
pub mod conversions;
pub mod error;

pub fn parse(file_meta: Arc<FileMeta>) -> Result<Option<Node>, Box<dyn Error>> {
    let span = span!(Level::INFO, "Parsing");
    let _guard = span.enter();
    let file_handle = match File::open(&file_meta.path) {
        Ok(f) => f,
        Err(e) => return Err(e.into())
    };
    let mut reader = BufReader::new(file_handle);

    match file_meta.path.extension().and_then(OsStr::to_str) {
        Some(ext) => match ext {
            "kml" => Ok(Some(Node {
                file: file_meta,
                metadata: parse_kml(&mut reader)?.into()
            })),
            "tif" => Ok(Some(Node {
                file: file_meta,
                metadata: parse_tiff(&mut reader)?.into()
            })),
            "dt1" | "dt2" => Ok(Some(Node {
                file: file_meta,
                metadata: parse_dt2(&mut reader)?.into()
            })),
            "geojson" => Ok(Some(Node {
                file: file_meta,
                metadata: parse_geojson(&mut reader)?.into()
            })),
            _ => {
                event!(Level::WARN, "Encountered unsupported file type: {ext}, ignoring!");
                return Ok(None); // Ignore as unsupported file type!
            }
        },
        None => {
            event!(Level::ERROR, "Error encountered parsing OsString into literal");
            return Err(ParseErrorKind::UnparseableExtension.into());
        }
    }
}

