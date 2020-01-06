use crate::dsp::types::*;
use sdl2::pixels::Color as SdlColor;

pub const WHITE: Color = Color {
    r: 255,
    g: 255,
    b: 255,
};
pub const BLACK: Color = Color { r: 0, g: 0, b: 0 };
pub const BLUE: Color = Color { r: 0, g: 0, b: 255 };
pub const LIGHT_BLUE: Color = Color {
    r: 200,
    g: 200,
    b: 255,
};
pub const GREEN: Color = Color { r: 0, g: 255, b: 0 };
pub const RED: Color = Color { r: 255, g: 0, b: 0 };
pub const GREY: Color = Color {
    r: 50,
    g: 50,
    b: 50,
};

pub const GRAY: Color = Color {
    r: 50,
    g: 50,
    b: 50,
};

pub const GRID_GRAY: Color = Color {
    r: 0xEE,
    g: 0xEE,
    b: 0xEE,
};

pub const BACKGROUND: Color = Color {
    r: 250,
    g: 250,
    b: 250,
};

pub const CURSOR_LIGHT: Color = Color {
    r: 190,
    g: 190,
    b: 190,
};
pub const CURSOR_DARK: Color = Color {
    r: 23,
    g: 23,
    b: 23,
};

impl Color {
    pub fn as_sdl(&self) -> SdlColor {
        SdlColor::RGB(self.r, self.g, self.b)
    }
}
