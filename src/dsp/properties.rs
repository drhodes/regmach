use crate::dsp::colors;
use crate::dsp::types::*;

impl DisplayProperties {
    pub fn new() -> DisplayProperties {
        DisplayProperties {
            current_color: colors::BACKGROUND,
            mouse_loc: DspPoint::new(0, 0),
            zoom: 0,
            frame: 0,
        }
    }
}
