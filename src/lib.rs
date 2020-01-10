#![allow(dead_code)]
#![allow(unused_must_use)]
#![feature(box_syntax)]

pub mod linux;

pub mod dsp {
    pub mod colors;
    pub mod dsp_point;
    pub mod linux_display;
    pub mod properties;
    pub mod segment;
    pub mod types;

    pub mod fragment_shaders;
    pub mod vertex_shaders;
}

pub mod schem {
    pub mod schem;
    pub mod types;
    pub mod wire;
}
