/// A DspPoint Display Point in display space, with screen
/// coordinates, y-pos points down and x-pos points right.
#[derive(Clone, Debug)]
pub struct DspPoint {
    pub x: i32,
    pub y: i32,
}

pub struct Err(String);

#[derive(Clone, Debug)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EntityId(pub u32);

pub struct WorldPoint {
    pub x: f32,
    pub y: f32,
}

pub struct BoundingBox {
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32,
}

pub trait Entity {
    fn bounding_box(self: &Self) -> BoundingBox;
    fn entity_id(self: &Self) -> EntityId;
    fn draw(self: &Self, dsp: &mut Display);
}

/// High level interface commands supported by Display.

// Maybe command should instead be REQUEST, and also have a RESPONSE.
#[derive(Debug)]
pub enum Command {
    /// Add a
    //AddButton(EntityId, Icon,)
    AddBox(f32, f32, f32, f32),
    AddText(f32, f32, String),
    AddMesh(Vec<f32>, Vec<u16>),
    SetStrokeSize(u32),
    SetDrawColor(Color),
    SetSnap(EntityId, bool), // default true
    // UserDialog(Dialog) -> RESPONSE.
    Batch(Vec<Command>),
}

/// These events are sent from display to schematic.
#[derive(Debug)]
pub enum Event {
    Quit,
    MouseUp(DspPoint),
    MouseDown(DspPoint),
    MouseDrag(DspPoint),
    MouseMove(DspPoint),
    KeyDown(u32),
}

pub struct DisplayProperties {
    pub current_color: Color,
    pub zoom: f32,
    pub mouse_loc: DspPoint,
    pub frame: u64,
}

/// Display is a baDspPointand driven graphics provider.  It's an
/// interface to the lower level canvas on whatever platform.
/// Entities don't know about the platform, but they know how to build
/// commands that the Display knows how to interpret.
pub trait Display {
    fn exec(self: &mut Self, cmd: &Command);
    fn exec_cmds(self: &mut Self, cmds: Vec<Command>);
    fn get_events(self: &mut Self) -> Vec<Event>;
    // fn undo
    // fn redo
}
