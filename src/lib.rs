#![allow(dead_code)]
#![allow(unused_must_use)]
#![feature(box_syntax)]

pub mod linux;

pub mod dsp {
    pub mod colors;
    pub mod linux_display;
    pub mod segment;
    pub mod types;
}

pub mod schem {
    pub mod schem;
    pub mod types;
    pub mod wire;
}
