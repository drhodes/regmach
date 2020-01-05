/// A DspPoint Display Point in display space, with screen
/// coordinates, y-pos points down and x-pos points right.
#[derive(Clone)]
pub struct DspPoint {
    pub x: i32,
    pub y: i32,
}

///
#[derive(Clone)]
pub struct Segment {
    pub p1: DspPoint,
    pub p2: DspPoint,
}

pub struct Err(String);

pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

/// High level interface commands supported by Display. The display has
/// no memory (for now), so each frame is repainted.  This could prove
/// to be slow, and should be one of the first things to optimize.
pub enum Command<'a> {
    /// Add a
    AddSegment(Segment),
    AddText(i16, i16, String),
    SetStrokeSize(f32),
    SetDrawColor(&'a Color),
    FilledCircle(DspPoint, u32), // center, radius
    Circle(DspPoint, u32),       // center, radius
    FillScreen,
    //Redraw,
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
    pub current_color: sdl2::pixels::Color,
    pub ctx: sdl2::Sdl,
    pub canvas: sdl2::render::Canvas<sdl2::video::Window>,
    pub zoom: i32,
}

// -----------------------------------------------------------------------------

// pub struct App {
//     display: dyn Display,
// }
