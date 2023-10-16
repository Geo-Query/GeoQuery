use std::any::Any;
use std::iter::Map;
use std::path::PathBuf;
use crate::kml::parse_kml_file;
use crate::region::Region;

pub fn draw_boundaries(coordinates: Vec<[f64; 2]>) -> ([f64; 2], [f64;2]) {
    let mut min_x: f64 = coordinates[0][0];
    let mut min_y: f64 = coordinates[0][1];
    let mut max_x: f64 = coordinates[0][0];
    let mut max_y: f64 = coordinates[0][1];
    for coordinate in coordinates {
        if coordinate[0] > max_x {
            max_x = coordinate[0];
        }
        if coordinate[0] < min_x {
            min_x = coordinate[0];
        }
        if coordinate[1] > max_y {
            max_y = coordinate[1];
        }
        if coordinate[1] < min_y {
            min_y = coordinate[1];
        }
    }
    return ([min_x, min_y], [max_x, max_y]);
}


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
    pub fn from_file_handle(handle: MapFileDescriptor) -> MapData {
        match handle.file_type {
            FileType::KML => {
                return parse_kml_file(handle);
            }
        }
    }
}