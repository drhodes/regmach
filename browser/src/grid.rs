use crate::types::*;
use wasm_bindgen::prelude::*;
use web_sys;
use web_sys::WebGl2RenderingContext;

// this should be able to be done completely in a shader.

impl Grid {
    pub fn new(dsp: &BrowserDisplay) -> Result<Grid, JsValue> {
        let meshes = vec!(Grid::make_grid_mesh(dsp, 1, 1024)?,
                          Grid::make_grid_mesh(dsp, 2, 1024)?,
                          Grid::make_grid_mesh(dsp, 4, 1024)?);
        Ok(Grid{meshes})
    }

    pub fn make_grid_mesh(dsp: &BrowserDisplay, level_of_detail: usize, extent: i32) -> Result<Mesh, String>  {
        let mut verts: Vec<f32> = vec![];
        
        for i in (-extent .. extent).step_by(level_of_detail) {            
            let (x1, y1, z1) = (i as f32, -extent as f32, 0.0001);
            let (x2, y2, z2) = (i as f32, extent as f32, 0.0001);
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
        
        Mesh::from_verts(
            &dsp,
            verts,
            include_str!("../shaders/grid-shader.vs"),
            include_str!("../shaders/grid-shader.fs"),
        )
    }
    
    pub fn draw(&self, dsp: &BrowserDisplay) {
        log!("zoom: {:?}", dsp.camera.pos.z);
        let zoom = dsp.camera.pos.z.abs();
        
        if zoom < 36.0 {
            self.meshes[0].draw_with_mode(dsp, WebGl2RenderingContext::LINES);
        } else if zoom < 100.0 {
            self.meshes[1].draw_with_mode(dsp, WebGl2RenderingContext::LINES);
        } else  {
            self.meshes[2].draw_with_mode(dsp, WebGl2RenderingContext::LINES);
        }
    }
}
