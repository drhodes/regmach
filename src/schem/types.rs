use crate::dsp::types::*

pub enum Value {
    Label(String),
    Str(String),
    Int(i32),
}

pub struct Button {
    text: String,
}

pub struct Register {
    value: Value,
}

pub struct BBox {
    top_left: DspPoint,
    bottom_right: DspPoint,
}

pub struct Wire {
    segments: Vec<Segment>,
}

pub struct Module {
    entities: Vec<Entity>,
}


pub trait Entity {
    fn point_inside(self: &Self, p: &DspPoint) -> bool;
    fn bounding_box(self: &Self) -> BBox;
    fn draw_commands(self: &Self) -> Vec<Command>;
}

pub struct Schematic {
    fn add_entity(e: dyn Entity) -> Err;
}
