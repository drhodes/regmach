// #![allow(dead_code)]
// #![allow(unused_must_use)]
// #![allow(unused_imports)]
// #![allow(unused_variables)]

#![allow(warnings)]
#![feature(box_syntax)]
#![cfg_attr(rustfmt, rustfmt_skip)]


// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

pub mod browser_display;
pub mod gl_util;
pub mod main_func;
pub mod start;
pub mod types;
pub mod camera;
pub mod mesh;
pub mod grid;
pub mod font_mesh;
pub mod text;
pub mod space_hash;
pub mod font_mgr;
pub mod button;
