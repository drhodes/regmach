use crate::gl_util;
use crate::types::*;
use nalgebra_glm as glm;
use regmach::dsp::types as rdt;
use regmach::dsp::types::Display;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys;
use web_sys::WebGl2RenderingContext as GL;
use web_sys::WebGlBuffer;

impl Mesh {
    pub fn from_verts(dsp: &BrowserDisplay,
                      verts: Vec<f32>,
                      vert_shadertxt: &str,
                      frag_shadertxt: &str)
                      -> Result<Mesh, String> {
        let vert_shader = Mesh::load_vertex_shader(dsp, vert_shadertxt)?;
        let frag_shader = Mesh::load_fragment_shader(dsp, frag_shadertxt)?;
        let program = gl_util::link_program(&dsp.ctx, &vert_shader, &frag_shader)?;
        let buffer: WebGlBuffer = dsp.ctx.create_buffer().ok_or("failed to create buffer")?;

        // todo. determine if scaling meshes will be possible.  if so
        // then they will need to be centered around the origin before
        // hand.  then the position transformation happens otherwise
        // the scaling matrix will change the position.

        let mesh = Mesh { vertices: verts,
                          shader_program: program,
                          vertex_buffer: buffer,
                          x: 0.0,
                          y: 0.0,
                          translation_matrix: glm::Mat4::identity() };

        dsp.ctx.use_program(Some(&mesh.shader_program));
        dsp.ctx.bind_buffer(GL::ARRAY_BUFFER, Some(&mesh.vertex_buffer));

        unsafe {
            // Note that `Float32Array::view` is somewhat dangerous
            // (hence the `unsafe`!). This is creating a raw view into
            // our module's `WebAssembly.Memory` buffer, but if we
            // allocate more pages (aka do a memory allocation in
            // Rust) it'll cause the buffer to change, causing the
            // `Float32Array` to be invalid.
            //
            // As a result, after `Float32Array::view` we have to be
            // very careful not to do any memory allocations before
            // it's dropped.
            let vert_array = js_sys::Float32Array::view(&mesh.vertices);

            dsp.ctx.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &vert_array, GL::STATIC_DRAW);
        }

        dsp.ctx.vertex_attrib_pointer_with_i32(0, 3, GL::FLOAT, false, 0, 0);
        dsp.ctx.enable_vertex_attrib_array(0);
        Ok(mesh)
    }

    pub fn move_to(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;
        self.translation_matrix = glm::translation(&glm::TVec3::new(-x, y, 0.0));
    }

    /// Mode is LINES, TRIANGLE, TRIANGLESTRIP, etc.
    pub fn draw_with_mode(&self, dsp: &BrowserDisplay, mode: u32) {
        dsp.ctx.use_program(Some(&self.shader_program));
        dsp.ctx.bind_buffer(GL::ARRAY_BUFFER, Some(&self.vertex_buffer));
        dsp.ctx.vertex_attrib_pointer_with_i32(0, 3, GL::FLOAT, false, 0, 0);

        let uniform_loc = dsp.ctx.get_uniform_location(&self.shader_program, "mvp");
        let is_transposed = false;
        let mut m = dsp.camera.get_view_projection();

        // negate the y component to
        m = m * self.translation_matrix;

        dsp.ctx.uniform_matrix4fv_with_f32_array(uniform_loc.as_ref(), is_transposed, m.as_slice());
        dsp.ctx.draw_arrays(mode, 0, (self.vertices.len() / 3) as i32);
    }

    pub fn load_vertex_shader(dsp: &BrowserDisplay, src: &str) -> Result<web_sys::WebGlShader, String> {
        gl_util::compile_shader(&dsp.ctx, GL::VERTEX_SHADER, src)
    }

    pub fn load_fragment_shader(dsp: &BrowserDisplay, src: &str) -> Result<web_sys::WebGlShader, String> {
        gl_util::compile_shader(&dsp.ctx, GL::FRAGMENT_SHADER, src)
    }
}
