use std::cell::RefCell;
use std::rc::Rc;
use web_sys;
use web_sys::{WebGl2RenderingContext, WebGlBuffer, WebGlProgram};

use nalgebra_glm as glm;

use regmach::dsp::types as rdt;
use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash)]
pub struct MeshId(u32);

pub struct Mesh {
    pub vertices: Vec<f32>,
    // pub indices: Vec<u16>,
    pub shader_program: WebGlProgram,
    pub vertex_buffer: WebGlBuffer,
}

pub struct FontMesh {
    pub vertices: Vec<f32>,
    pub colors: Vec<f32>,
    // pub indices: Vec<u16>,
    pub shader_program: WebGlProgram,
    pub vertex_buffer: WebGlBuffer,
    pub color_buffer: WebGlBuffer,
}

pub struct BrowserDisplay {
    pub window: web_sys::Window,
    pub canvas: web_sys::HtmlCanvasElement,
    pub wrapper: web_sys::HtmlDivElement,
    pub ctx: WebGl2RenderingContext,
    pub events: Rc<RefCell<Vec<rdt::Event>>>,
    pub props: rdt::DisplayProperties,
    pub camera: Camera,
    pub(super) mesh_store: HashMap<MeshId, Mesh>,
    pub(super) mesh_nonce: u32,
}

pub type V3 = glm::Vec3;

pub struct Camera {
    pub pos: V3,
    pub fov: f32,
    pub aspect: f32,
    pub z_near: f32,
    pub z_far: f32,
    pub perspective: glm::Mat4x4,
    pub forward: V3,
    pub up: V3,
}

pub struct Grid {
    pub meshes: Vec<Mesh>,
}

// pub struct CompoundMesh {
//     pub meshes: Vec<Mesh>,
// }

pub struct Text {
    pub color: rdt::Color,
    pub text: String,
    pub font_mesh: FontMesh,
}

// some small numbers
pub const EPSILON32: f32 = 1e-12;
pub const EPSILON64: f64 = 1e-12;
