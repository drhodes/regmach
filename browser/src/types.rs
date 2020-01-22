use std::cell::RefCell;
use std::rc::Rc;
use web_sys::{WebGl2RenderingContext, WebGlBuffer, WebGlProgram};
use web_sys;

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

pub struct BrowserDisplay {
    pub canvas: web_sys::HtmlCanvasElement,
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
    pub perspective: glm::Mat4x4,
    pub forward: V3,
    pub up: V3,
}

pub struct Grid {
    pub mesh: Mesh,
}
