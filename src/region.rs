use rstar::{AABB, RTreeObject};

pub struct Region {
    pub(crate) top_left: [i32; 2],
    pub(crate) bottom_right: [i32; 2],
}

pub struct RegionNode {
    pub region: Region,
    pub value: i32 // TODO: Somehow define this reference?
}

impl RTreeObject for RegionNode {
    type Envelope = AABB<[i32; 2]>;

    fn envelope(&self) -> Self::Envelope {
        AABB::from_corners(self.region.top_left, self.region.bottom_right)
    }
}