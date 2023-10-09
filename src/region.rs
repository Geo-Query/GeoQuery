use rstar::{AABB, RTreeObject};
use crate::data::MapData;

pub struct Region {
    top_left: [i32; 2],
    bottom_right: [i32; 2],
}

pub struct RegionNode {
    pub region: Region,
    value: i32 // TODO: Somehow define this reference?
}

impl RTreeObject for RegionNode {
    type Envelope = AABB<[i32; 2]>;

    fn envelope(&self) -> Self::Envelope {
        AABB::from_corners(self.region.top_left, self.region.bottom_right)
    }
}