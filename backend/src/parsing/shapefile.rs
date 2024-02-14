use std::error::Error;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::{BufReader, ErrorKind, Read};
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use crate::parsing::error::ParseErrorKind::UnparseableExtension;
use crate::spatial::Region;

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
    println!("We parsing headers!");
    let tags = vec![("Filetype".to_string(), "SHAPEFILE".to_string())];
    let mut header_buf = [0u8; 100];
    shp_reader.read_exact(&mut header_buf)?;
    let header = parse_header(&header_buf)?;

    if let Some(prj_reader) = prj_reader {
        let
    }

    return Err(UnparseableExtension.into());

}