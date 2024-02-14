use std::error::Error;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::{BufReader, ErrorKind, Read};
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use crate::spatial::Region;
use proj4rs::Proj;
use proj4rs::proj::ProjType;
use proj4wkt::wkt_to_projstring;
use tracing::{event, Level};


pub trait FromBytes {
    fn from_bytes(bytes: &[u8]) -> Self;
}

impl FromBytes for f64 {
    fn from_bytes(bytes: &[u8]) -> Self {
        let bytes: [u8; 8] = bytes.try_into().unwrap();
        return f64::from_le_bytes(bytes);
    }
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShapeFileMap {
    pub shp: PathBuf,
    pub prj: Option<PathBuf>,
    pub tfw: Option<PathBuf>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShapeFileMetaData {
    pub region: Region,
    pub tags: Vec<(String, String)>
}

#[derive(Debug)]
pub enum ShapeFileErrorKind {
    UnexpectedMagicNumber([u8; 4])
}

impl Display for ShapeFileErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl Error for ShapeFileErrorKind {}

#[derive(Debug)]
pub struct ShapeFileHeader {
    x_min: f64,
    y_min: f64,
    x_max: f64,
    y_max: f64
}

pub fn parse_header(buffer: &[u8]) -> Result<ShapeFileHeader, Box<dyn Error>> {
    assert_eq!(buffer.len(), 100); // Assert valid slice! Runtime check!
    if buffer[0..4] != [0,0,39,10] {
        return Err(ShapeFileErrorKind::UnexpectedMagicNumber([buffer[0], buffer[1], buffer[2], buffer[3]]).into());
    }
    let x_min = f64::from_bytes(&buffer[36..44]);
    let y_min = f64::from_bytes(&buffer[44..52]);
    let x_max = f64::from_bytes(&buffer[52..60]);
    let y_max = f64::from_bytes(&buffer[60..68]);

    return Ok(ShapeFileHeader {
        x_min,
        y_min,
        x_max,
        y_max,
    })
}

pub fn parse_shapefile(shp_reader: &mut BufReader<File>, prj_reader: Option<&mut BufReader<File>>) -> Result<ShapeFileMetaData,Box<dyn Error>> {
    let to_proj = Proj::from_proj_string(crs_definitions::EPSG_4326.proj4).expect("FAILED TO BUILD DEFAULT PROJ!");
    let tags = vec![("Filetype".to_string(), "SHAPEFILE".to_string())];
    let mut header_buf = [0u8; 100];
    shp_reader.read_exact(&mut header_buf)?;
    let header = parse_header(&header_buf)?;

    if let Some(prj_reader) = prj_reader {
        let mut prj_content = String::new();
        prj_reader.read_to_string(&mut prj_content)?;
        let proj = Proj::from_proj_string(wkt_to_projstring(prj_content.as_str())?.as_str())?;
        let (mut top_left, mut bottom_right) = match proj.projection_type() {
            ProjType::Latlong => ((header.x_min.to_radians(), header.y_max.to_radians()), (header.x_max.to_radians(), header.y_min.to_radians())),
            ProjType::Other => ((header.x_min, header.y_max), (header.x_max, header.y_min)),
            ProjType::Geocentric => {
                event!(Level::ERROR, "Unsupported projection! From GEOCENTRIC!");
                event!(Level::ERROR, "Please contact developer, and send file content for implementation.");
                panic!();
            }
        };


        event!(Level::INFO, "Applying Projection to {top_left:?} and {bottom_right:?}");
        proj4rs::transform::transform(&proj, &to_proj, &mut top_left)?;
        proj4rs::transform::transform(&proj, &to_proj, &mut bottom_right)?;
        event!(Level::INFO, "Parsed shapefile and applied projection!");
    } else {
        event!(Level::WARN, "Shapefile without accompanying projection found!");
        event!(Level::WARN, "This is a forseen error, and we will assume that the CRS is EPSG:4326!");
        event!(Level::WARN, "However, this might be incorrect! If you encounter inaccuracies in shapefile");
        event!(Level::WARN, "Please contact the developers, and attach the unhandled file!")
        // TODO: Add a config option to disable this behaviour!
    }



    println!("{:?}", header);
    let mut top_left = (header.x_min.to_radians(), header.y_max.to_radians());
    let mut bottom_right = (header.x_max.to_radians(), header.y_min.to_radians(),);


    return Ok(ShapeFileMetaData {
        region: Region { top_left: (top_left.0.to_degrees(), top_left.1.to_degrees()), bottom_right: (bottom_right.0.to_degrees(), bottom_right.1.to_degrees()) },
        tags,
    })
}