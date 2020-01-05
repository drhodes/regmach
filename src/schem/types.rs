use crate::dsp::types::*;

pub enum Value {
    Label(String),
    Str(String),
    Int(i32),
    Bool,
}

pub struct Button {
    pub text: String,
}

pub struct Register {
    pub value: Value,
}

pub struct BBox {
    pub top_left: DspPoint,
    pub bottom_right: DspPoint,
}

pub struct Wire {
    pub segments: Vec<Segment>,
}

// pub struct Module {
//     entities: Vec<dyn Entity>,
// }

pub trait Entity {
    // fn point_inside(self: &Self, p: &DspPoint) -> bool;
    // fn bounding_box(self: &Self) -> BBox;
    fn render(self: &Self) -> Vec<Command>;
}

pub struct Schematic {
    pub entities: Vec<Box<dyn Entity>>, //fn add_entity(e: dyn Entity) -> Err;
}
