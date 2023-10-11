mod region;
mod data;
use rstar::RTree;

struct State {
    pub index: RTree<region::Region>,
    pub data: Vec<data::MapData>
}

fn main() {
	// Not yet implemented.
    let state = State { index: Default::default(), data: vec![] };
}
