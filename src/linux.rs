use crate::dsp;
use crate::dsp::fragment_shaders::*;
use crate::dsp::types::*;
use crate::dsp::vertex_shaders::*;
use crate::schem::types::*;

use gl;
use gl::types::*;
use glfw::{Action, Context, Key, MouseButton};
use std::ffi::CString;
use std::mem;
use std::os::raw::c_void;
use std::ptr;
use std::str;
use std::time::Duration;

use lyon::math::{point, Point};
use lyon::path::builder::*;
use lyon::path::Path;
use lyon::tessellation::basic_shapes::fill_circle;
use lyon::tessellation::*;
use lyon_svg;

use rand::Rng;

pub fn start() {
    LinuxDisplayOpenGl::new();
}
