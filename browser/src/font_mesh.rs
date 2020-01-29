use crate::gl_util;
use crate::types::*;
use nalgebra_glm as glm;
use web_sys;
use web_sys::{WebGl2RenderingContext, WebGlBuffer};
use WebGl2RenderingContext as GL;

impl FontMesh {
    pub fn from_verts(dsp: &BrowserDisplay,
                      verts: Vec<f32>,
                      colors: Vec<f32>,
                      vert_shadertxt: &str,
                      frag_shadertxt: &str)
                      -> Result<FontMesh, String> {
        let vert_shader = Mesh::load_vertex_shader(dsp, vert_shadertxt)?;
        let frag_shader = Mesh::load_fragment_shader(dsp, frag_shadertxt)?;
        let program = gl_util::link_program(&dsp.ctx, &vert_shader, &frag_shader)?;
        let vertex_buffer: WebGlBuffer = dsp.ctx.create_buffer().ok_or("failed to create buffer")?;
        let color_buffer: WebGlBuffer = dsp.ctx.create_buffer().ok_or("failed to create buffer")?;

        // todo. determine if scaling meshes will be possible.  if so
        // then they will need to be centered around the origin before
        // hand.  then the position transformation happens otherwise
        // the scaling matrix will change the position.

        let mesh = FontMesh { vertices: verts,
                              colors: colors,
                              shader_program: program,
                              vertex_buffer: vertex_buffer,
                              color_buffer: color_buffer,
                              x: 0.0,
                              y: 0.0,
                              translation_matrix: glm::Mat4::identity() };

        dsp.ctx.use_program(Some(&mesh.shader_program));

        // VERTEX BUFFER
        dsp.ctx.bind_buffer(GL::ARRAY_BUFFER, Some(&mesh.vertex_buffer));
        unsafe {
            let vert_array = js_sys::Float32Array::view(&mesh.vertices);
            dsp.ctx.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &vert_array, GL::STATIC_DRAW);
        }
        dsp.ctx.vertex_attrib_pointer_with_i32(0, 3, GL::FLOAT, false, 0, 0);
        dsp.ctx.enable_vertex_attrib_array(0);

        // COLOR BUFFER
        dsp.ctx.bind_buffer(GL::ARRAY_BUFFER, Some(&mesh.color_buffer));
        unsafe {
            let color_array = js_sys::Float32Array::view(&mesh.colors);
            dsp.ctx.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &color_array, GL::STATIC_DRAW);
        }
        dsp.ctx.vertex_attrib_pointer_with_i32(1, 4, GL::FLOAT, false, 0, 0);
        dsp.ctx.enable_vertex_attrib_array(1);

        Ok(mesh)
    }

    /// Mode is LINES, TRIANGLE, TRIANGLESTRIP, etc.
    pub fn draw_with_mode(&self, dsp: &BrowserDisplay, mode: u32) {
        dsp.ctx.use_program(Some(&self.shader_program));

        dsp.ctx.bind_buffer(GL::ARRAY_BUFFER, Some(&self.vertex_buffer));
        dsp.ctx.vertex_attrib_pointer_with_i32(0, 3, GL::FLOAT, false, 0, 0);
        dsp.ctx.bind_buffer(GL::ARRAY_BUFFER, Some(&self.color_buffer));
        dsp.ctx.vertex_attrib_pointer_with_i32(1, 4, GL::FLOAT, false, 0, 0);

        let uniform_loc = dsp.ctx.get_uniform_location(&self.shader_program, "mvp");
        let is_transposed = false;

        let mut m = dsp.camera.get_view_projection();
        m *= self.translation_matrix;

        dsp.ctx.uniform_matrix4fv_with_f32_array(uniform_loc.as_ref(), is_transposed, m.as_slice());
        dsp.ctx.draw_arrays(mode, 0, (self.vertices.len() / 3) as i32);
    }

    pub fn load_vertex_shader(dsp: &BrowserDisplay, shadertxt: &str) -> Result<web_sys::WebGlShader, String> {
        gl_util::compile_shader(&dsp.ctx, GL::VERTEX_SHADER, shadertxt)
    }

    pub fn load_fragment_shader(dsp: &BrowserDisplay,
                                shadertxt: &str)
                                -> Result<web_sys::WebGlShader, String> {
        gl_util::compile_shader(&dsp.ctx, GL::FRAGMENT_SHADER, shadertxt)
    }

    pub fn move_to(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;
        self.translation_matrix = glm::translation(&glm::TVec3::new(-x, y, 0.0));
    }
}
