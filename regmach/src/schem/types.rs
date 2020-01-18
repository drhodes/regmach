use crate::dsp::types as dsp;

pub enum Value {
    Label(String),
    Str(String),
    Int(i32),
    Bool,
}

// course grid
pub struct Pos {
    x: i32,
    y: i32,
}

type EntityId = u32;

pub struct Button {
    pub id: EntityId,
    pub text: String,
}

pub struct Register {
    pub id: EntityId,
    pub value: Value,
}

pub struct Terminal {
    pub id: EntityId,
}

pub struct UserModule {
    pub name: String,
    pub terms: Vec<Terminal>,
}

pub struct Wire {
    pub id: EntityId,
    pub segments: Vec<dsp::Segment>,
}

pub trait Entity {
    // fn point_inside(self: &Self, p: &DspPoint) -> bool;
    // fn bounding_box(self: &Self) -> BBox;
    fn render(self: &Self) -> Vec<dsp::Command>;
}

// pub struct ToolBar {
// }

pub struct Schematic {
    pub entities: Vec<Box<dyn Entity>>, //fn add_entity(e: dyn Entity) -> Err;
}
