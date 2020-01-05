use crate::dsp::types::*;
use crate::schem::types::*;

impl Wire {
    pub fn new(segs: Vec<Segment>) -> Wire {
        Wire { segments: segs }
    }
}

impl Entity for Wire {
    // fn point_inside(self: &Self, p: &DspPoint) -> bool;
    // fn bounding_box(self: &Self) -> BBox;
    fn render(self: &Self) -> Vec<Command> {
        let mut cmds = vec![];
        for seg in &self.segments {
            cmds.push(Command::AddSegment(seg.clone()))
        }
        cmds
    }
}
