use regmach::dsp::types as rdt;
use std::cell::RefCell;
use std::rc::Rc;
use web_sys::{WebGlRenderingContext, WebGlBuffer, WebGlProgram};
use web_sys;
use nalgebra_glm as glm;
use crate::gl_util;

pub struct BrowserDisplay {
    pub canvas: web_sys::HtmlCanvasElement,
    pub ctx: WebGlRenderingContext,
    pub events: Rc<RefCell<Vec<rdt::Event>>>,
    pub props: rdt::DisplayProperties,
}

pub type V3 = glm::Vec3;

pub struct Camera {
    pub pos: V3,
    pub perspective: glm::Mat4x4,
    pub forward: V3,
    pub up: V3,
}

//pub type Vertex = [f32; 3];

pub struct Mesh {
    pub vertices: Vec<f32>,
    // pub indices: Vec<u16>,
    pub shader_program: WebGlProgram,
    pub vertex_buffer: WebGlBuffer,
}

impl Mesh {
    pub fn from_verts(dsp: &BrowserDisplay, verts: Vec<f32>) -> Result<Mesh, String>{
        let vert_shader = Mesh::load_vertex_shader(dsp)?;
        let frag_shader = Mesh::load_fragment_shader(dsp)?;
        let program = gl_util::link_program(&dsp.ctx, &vert_shader, &frag_shader)?;
        let buffer: WebGlBuffer = dsp.ctx.create_buffer().ok_or("failed to create buffer")?;
        
        let mesh = Mesh {
            vertices: verts,
            shader_program: program,
            vertex_buffer: buffer,
        };
        
        dsp.ctx.use_program(Some(&mesh.shader_program));
        dsp.ctx.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&mesh.vertex_buffer));

        unsafe {
            // Note that `Float32Array::view` is somewhat dangerous (hence the
            // `unsafe`!). This is creating a raw view into our module's
            // `WebAssembly.Memory` buffer, but if we allocate more pages for ourdsp
            // (aka do a memory allocation in Rust) it'll cause the buffer to change,
            // causing the `Float32Array` to be invalid.
            //
            // As a result, after `Float32Array::view` we have to be very careful not to
            // do any memory allocations before it's dropped.
            let vert_array = js_sys::Float32Array::view(&mesh.vertices);
            dsp.ctx.buffer_data_with_array_buffer_view(
                WebGlRenderingContext::ARRAY_BUFFER,
                &vert_array,
                WebGlRenderingContext::STATIC_DRAW,
            );
        }
        
        dsp.ctx.vertex_attrib_pointer_with_i32(0, 3, WebGlRenderingContext::FLOAT, false, 0, 0);
        dsp.ctx.enable_vertex_attrib_array(0);
        Ok(mesh)
    }

    pub fn draw(&self, dsp: &BrowserDisplay) {
        let n = (dsp.props.frame % 255) as f32;
        let c = n / (255 as f32);
        
        dsp.ctx.clear_color(c, 1.0 - c, c, 1.0);
        dsp.ctx.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);
        dsp.ctx.draw_arrays(
            WebGlRenderingContext::TRIANGLES,
            0,
            (self.vertices.len() / 3) as i32,
        );
    }

    pub fn load_vertex_shader(dsp: &BrowserDisplay) -> Result<web_sys::WebGlShader, String> {
        gl_util::compile_shader(
            &dsp.ctx,
            WebGlRenderingContext::VERTEX_SHADER,
            r#"
            attribute vec4 position;
            void main() {
            gl_Position = position;
            }
            "#,
        )
    }

    pub fn load_fragment_shader(dsp: &BrowserDisplay) -> Result<web_sys::WebGlShader, String> {
        gl_util::compile_shader(
            &dsp.ctx,
            WebGlRenderingContext::FRAGMENT_SHADER,
            r#"
            void main() {
                gl_FragColor = vec4(1.0, 0.0, 1.0, 1.0);
            }
             "#,
        ) 
    }

    
}
