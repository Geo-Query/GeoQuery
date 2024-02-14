use std::error::Error;
use std::fs::File;
use std::io::{BufReader, ErrorKind};
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use crate::parsing::error::ParseErrorKind::UnparseableExtension;
use crate::spatial::Region;

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

pub fn parse_shapefile(shp_reader: &mut BufReader<File>, prj_reader: Option<&mut BufReader<File>>) -> Result<ShapeFileMetaData,Box<dyn Error>> {
    let tags = vec![("Filetype".to_string(), "SHAPEFILE".to_string())];

    return Err(UnparseableExtension.into());

}