use std::fmt::{Debug};
use std::sync::Arc;
use crate::spatial::{Coordinate, Region};
use rstar::{AABB, RTreeObject};
use serde::{Deserialize, Serialize};
use crate::MapType;

// Node
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Node {
    pub metadata: MetaData,
    pub map: Arc<MapType>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetaData {
    pub region: Region,
    pub tags: Vec<(String, String)>
}


// Implement RTreeObject on Node.
impl RTreeObject for Node {
    type Envelope = AABB<Coordinate>;

    fn envelope(&self) -> Self::Envelope {
        AABB::from_corners(self.metadata.region.top_left(), self.metadata.region.bottom_right())
    }
}