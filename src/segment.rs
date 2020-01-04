use crate::types::*;

impl Segment {
    pub fn from_coords(x1: i32, y1: i32, x2: i32, y2: i32) -> Segment {
        Segment {
            p1: ScmPoint { x: x1, y: y1 },
            p2: ScmPoint { x: x2, y: y2 },
        }
    }
}
