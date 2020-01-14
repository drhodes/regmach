// #![allow(dead_code)]
// #![allow(unused_must_use)]
// #![allow(unused_imports)]
// #![allow(unused_variables)]
#![allow(warnings)]

#![feature(box_syntax)]
#![cfg_attr(rustfmt, rustfmt_skip)]

pub mod linux;

pub mod dsp {
    pub mod colors;
    pub mod dsp_point;
    pub mod linux_display;
    pub mod linux_display_opengl;
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
