use std::any::Any;
use std::iter::Map;
use std::path::PathBuf;
use crate::kml::parse_kml_file;
use crate::region::Region;

pub enum FileType { // TODO: Implement readers for each filetype.
    // Started KML Parser
    KML,
}

pub struct MapFileDescriptor {
    pub file_type: FileType,
    pub file_path: PathBuf
}

pub struct MapData {
    pub handle: MapFileDescriptor,
    pub region: Region,
}

impl MapData {
    pub fn from_file_handle(handle: MapFileDescriptor) {
        match handle.file_type {
            FileType::KML => {
                parse_kml_file(handle);
            }
        }
    }
}