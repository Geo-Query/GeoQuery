use rstar::{AABB, RTreeObject};

#[derive(Debug)]
pub struct Region {
    pub(crate) bottom_left: [f64; 2],
    pub(crate) top_right: [f64; 2],
}

pub struct RegionNode {
    pub region: Region,
    pub value: i32 // TODO: Somehow define this reference?
}

impl RTreeObject for RegionNode {
    type Envelope = AABB<[f64; 2]>;

    fn envelope(&self) -> Self::Envelope {
        AABB::from_corners(self.region.bottom_left, self.region.top_right)
    }
}