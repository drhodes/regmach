use crate::gl_util;
use crate::types::*;
use nalgebra_glm as glm;
use wasm_bindgen::prelude::*;
use web_sys;
use web_sys::{WebGl2RenderingContext, WebGlBuffer};

// this should be able to be done completely in a shader.

impl Grid {
    pub fn new(dsp: &BrowserDisplay) -> Result<Grid, JsValue> {
        let mut verts: Vec<f32> = vec![];
        // here are 4000 verts, which
        for i in -1000..1000 {
            let (x1, y1, z1) = (i as f32, -1000.0, 0.1);
            let (x2, y2, z2) = (i as f32, 1000.0, 0.1);
            verts.push(x1);
            verts.push(y1);
            verts.push(z1);
            verts.push(x2);
            verts.push(y2);
            verts.push(z2);
            // flip em.
            verts.push(y1);
            verts.push(x1);
            verts.push(z1);
            verts.push(y2);
            verts.push(x2);
            verts.push(z2);
        }
        let mesh = Mesh::from_verts(
            &dsp,
            verts,
            include_str!("../shaders/grid-shader.vs"),
            include_str!("../shaders/grid-shader.fs"),
        )?;
        Ok(Grid { mesh })
    }
    pub fn draw(&self, dsp: &BrowserDisplay) {
        self.mesh.draw_with_mode(dsp, WebGl2RenderingContext::LINES);
    }
}
