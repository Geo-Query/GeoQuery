use rstar::{AABB, RTreeObject};

pub struct Region {
    top_left: [i32; 2],
    bottom_right: [i32; 2],
    value: i32 // For now is intended as arr index for arr of file handles.
    // TODO: Edit above type once a std handle type is implemented.
}

impl RTreeObject for Region {
    type Envelope = AABB<[i32; 2]>;

    fn envelope(&self) -> Self::Envelope {
        AABB::from_corners(self.top_left, self.bottom_right)
    }
}