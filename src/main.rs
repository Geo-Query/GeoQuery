mod region;
mod data;
mod kml;

use std::path::PathBuf;
use rstar::RTree;
use crate::data::MapData;

struct State {
    pub index: RTree<region::RegionNode>,
    pub data: Vec<data::MapData>
}

fn main() {
	// Not yet implemented.
    let state = State { index: Default::default(), data: vec![] };

    let kml_file = data::MapFileDescriptor {
        file_type: data::FileType::KML,
        file_path: PathBuf::from("/home/ben/uni/psd/teamproj/sh35-data-parsing/luciad_and_leuven.kml"),
    };
    MapData::from_file_handle(kml_file);
}
