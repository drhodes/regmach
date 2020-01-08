use crate::dsp::types::*;
use sdl2::pixels::Color as SdlColor;

const fn c(r: u8, g: u8, b: u8) -> Color {
    Color { r, g, b }
}

pub const WHITE: Color = c(255, 255, 255);
pub const BLACK: Color = c(0, 0, 0);
pub const BLUE: Color = c(0, 0, 255);
pub const LIGHT_BLUE: Color = c(200, 200, 255);
pub const GREEN: Color = c(0, 255, 0);
pub const RED: Color = c(255, 0, 0);
pub const GREY: Color = c(50, 50, 5);
pub const GRAY: Color = c(50, 50, 5);
pub const GRID_GRAY: Color = c(0xEE, 0xEE, 0xEE);
pub const BACKGROUND: Color = c(250, 250, 2);
pub const CURSOR_LIGHT: Color = c(190, 190, 190);
pub const CURSOR_DARK: Color = c(23, 23, 23);

impl Color {
    pub fn as_sdl(&self) -> SdlColor {
        SdlColor::RGB(self.r, self.g, self.b)
    }
    // pub fn as_gl(&self) -> SdlColor {
    //     SdlColor::RGB(self.r, self.g, self.b)
    // }
}
