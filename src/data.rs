use std::any::Any;
use std::iter::Map;
use std::path::PathBuf;
use crate::region::Region;

enum FileType {
    FooType // TODO: Implement readers for each filetype.
}

pub struct PlaceholderHandleType {
    pub file_type: FileType,
    pub file_path: PathBuf
}

pub struct MapData {
    pub handle: PlaceholderHandleType,
    pub region: Region,
}

impl MapData {
    fn from_file_handle(handle: PlaceholderHandleType) -> MapData {
        match handle.file_type {
            FileType::FooType => {
                MapData {
                    handle,
                    region: Region{
                        top_left: [-1,-1],
                        bottom_right: [0,0]
                    },
                }
            }
        }
    }
}