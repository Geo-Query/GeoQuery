use crate::index::Node;
use crate::parsing::dted::parse_dted;
use crate::parsing::geojson::parse_geojson;
use crate::parsing::gpkg::parse_gpkg;
use crate::parsing::kml::parse_kml;
use crate::parsing::mbtiles::parse_mbtiles;
use crate::MapType;
use geotiff::parse_tiff;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::sync::Arc;
use tracing::{span, Level};

use crate::parsing::shapefile::parse_shapefile;

pub mod dted;
pub mod geojson;
pub mod kml;

pub mod mbtiles;

pub mod conversions;
pub mod error;
pub(crate) mod gpkg;
pub(crate) mod shapefile;

pub fn parse(map: Arc<MapType>) -> Result<Option<Node>, Box<dyn Error>> {
    let span = span!(Level::INFO, "Parsing");
    let _guard = span.enter();

    match map.as_ref() {
        MapType::GEOTIFF(tiff) => Ok(Some(Node {
            metadata: parse_tiff(
                &mut BufReader::new(File::open(&tiff.tiff)?),
                tiff.tfw
                    .clone()
                    .map(File::open)
                    .transpose()?
                    .map(BufReader::new)
                    .as_mut(),
            )?
            .into(),
            map,
        })),
        MapType::DTED(dted) => Ok(Some(Node {
            metadata: parse_dted(&mut BufReader::new(File::open(&dted.path)?))?.into(),
            map,
        })),
        MapType::KML(kml) => Ok(Some(Node {
            metadata: parse_kml(&mut BufReader::new(File::open(&kml.path)?))?.into(),
            map,
        })),
        MapType::GEOJSON(geojson) => Ok(Some(Node {
            metadata: parse_geojson(&mut BufReader::new(File::open(&geojson.path)?))?.into(),
            map,
        })),
        MapType::MBTILES(mbtiles) => Ok(Some(Node {
            metadata: parse_mbtiles(&mbtiles.path.to_str().unwrap())?.into(),
            map,
        })),
        MapType::GPKG(gpkg) => Ok(Some(Node {
            metadata: parse_gpkg(&gpkg.path.to_str().unwrap())?.into(),
            map,
        })),
        MapType::SHAPEFILE(shapefile) => {
            let mut shp_reader = BufReader::new(File::open(&shapefile.shp)?);
            let mut prj_reader = shapefile
                .prj
                .clone()
                .map(File::open)
                .transpose()?
                .map(BufReader::new);

            return Ok(Some(Node {
                metadata: parse_shapefile(&mut shp_reader, prj_reader.as_mut())?.into(),
                map,
            }));
        }
    }
}
