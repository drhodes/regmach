/// A DspPoint Display Point in display space, with screen
/// coordinates, y-pos points down and x-pos points right.
#[derive(Clone, Debug)]
pub struct DspPoint {
    pub x: i32,
    pub y: i32,
}

///
#[derive(Clone, Debug)]
pub struct Segment {
    pub p1: DspPoint,
    pub p2: DspPoint,
}

pub struct Err(String);

#[derive(Clone, Debug)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

/// High level interface commands supported by Display. The display has
/// no memory (for now), so each frame is repainted.  This could prove
/// to be slow, and should be one of the first things to optimize.

// Maybe command should instead be REQUEST, and also have a RESPONSE.
#[derive(Debug)]
pub enum Command {
    /// Add a
    AddSegment(Segment),
    AddText(i16, i16, String),
    SetStrokeSize(u32),
    SetDrawColor(Color),
    FillScreen,
    RenderCursor,
    Zoom(i32),
    IncrementFrame,
    DrawGrid,
    // UserDialog(Dialog) -> RESPONSE.
}

#[derive(Debug)]
pub enum Event {
    Quit,
    MouseUp(DspPoint),
    MouseDown(DspPoint),
    MouseDrag(DspPoint),
    MouseMove(DspPoint),
    KeyDown(u32),
    DeviceClick(u32),
}

pub struct DisplayProperties {
    pub current_color: Color,
    pub zoom: f32,
    pub mouse_loc: DspPoint,
    pub frame: u64,
}

/// Display does not know about entities.  Display is a basic command
/// driven graphics provider.  It's an interface to the lower level
/// canvas on whatever platform.  Entities don't know about the
/// platform, but they know how to build commands that the Display
/// knows how to interpret.
pub trait Display {
    fn exec(self: &mut Self, cmd: &Command);
    fn exec_cmds(self: &mut Self, cmds: Vec<Command>);
    fn get_events(self: &mut Self) -> Vec<Event>;
}
