use crate::spatial::{Region, Coordinate};
use rstar::{RTreeObject, AABB};

// Node
#[derive(Debug)]
pub struct Node {
    pub region: Box<dyn Region>,
    pub map_data_index: i32
}

// Implement RTreeObject on Node.
impl RTreeObject for Node {
    type Envelope = AABB<Coordinate>;

    fn envelope(&self) -> Self::Envelope {
        AABB::from_corners(self.region.bottom_left(), self.region.top_right())
    }
}