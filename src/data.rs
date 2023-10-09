use std::path::PathBuf;
use crate::region::Region;

enum FileType {
    FooType // TODO: Implement readers for each filetype.
}

pub struct MapData {
    pub file_path: PathBuf,
    pub region: Region,
    pub file_type: FileType
}

