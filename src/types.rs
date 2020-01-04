/// A ScmPoint in schematic space. y-pos point up and x-pos points right,
/// everyone's favorite coordinate system
pub struct ScmPoint {
    pub x: i32,
    pub y: i32,
}

///
pub struct Segment {
    pub p1: ScmPoint,
    pub p2: ScmPoint,
}

pub struct Err(String);

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
    top_left: (i32, i32),
    bottom_right: (i32, i32),
}

pub struct Wire {
    segments: Vec<Segment>,
}

pub trait Entity {
    fn point_inside(self: &Self, p: &ScmPoint) -> bool;
    fn bounding_box(self: &Self) -> BBox;
    fn draw_commands(self: &Self) -> Vec<Command>;
}

pub trait Schematic {
    fn add_entity(e: dyn Entity) -> Err;
}

pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

pub enum Command<'a> {
    AddSegment(Segment),
    AddText(ScmPoint, String),
    SetStrokeSize(f32),
    SetDrawColor(&'a Color),
    FilledCircle(ScmPoint, u32), // center, radius
    Circle(ScmPoint, u32),       // center, radius
    FillScreen,
    Redraw,
    Zoom(i32),
}

/// Display does not know about entities.  Display is a basic command
/// driven graphics provider.  It's an interface to the lower level
/// canvas on whatever platform.  Entities don't know about the
/// platform, but they know how to build commands that the Display
/// knows how to interpret.
pub trait Display {
    fn exec(self: &mut Self, cmd: &Command);
    fn exec_cmds(self: &mut Self, cmds: Vec<Command>);
}

pub struct LinuxDisplay {
    pub ctx: sdl2::Sdl,
    pub canvas: u8, //sdl2::render::Canvas<sdl2::video::Window>,
    pub zoom: i32,
}

pub struct App {
    display: dyn Display,
}
