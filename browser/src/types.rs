//use glm;
use regmach::dsp::types as rdt;
use std::cell::RefCell;
use std::rc::Rc;
use web_sys::{WebGlRenderingContext, WebGlBuffer, WebGlProgram};
use web_sys;
use nalgebra_glm as glm;

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

pub type Vertex = [f32; 3];

pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u16>,
    pub shader_program: WebGlProgram,
    pub vertex_buffer: WebGlBuffer,
}


impl Mesh {
    // dsp.ctx.use_program(Some(self.&shader_program));
    // dsp.ctx.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&self.vertex_buffer));

    // unsafe {
    //     // Note that `Float32Array::view` is somewhat dangerous (hence the
    //     // `unsafe`!). This is creating a raw view into our module's
    //     // `WebAssembly.Memory` buffer, but if we allocate more pages for ourdsp
    //     // (aka do a memory allocation in Rust) it'll cause the buffer to change,
    //     // causing the `Float32Array` to be invalid.
    //     //
    //     // As a result, after `Float32Array::view` we have to be very careful not to
    //     // do any memory allocations before it's dropped.
    //     let vert_array = js_sys::Float32Array::view(&vertices);
    //     dsp.ctx.buffer_data_with_array_buffer_view(
    //         WebGlRenderingContext::ARRAY_BUFFER,
    //         &vert_array,
    //         WebGlRenderingContext::STATIC_DRAW,
    //     );
    // }

    // dsp.ctx.vertex_attrib_pointer_with_i32(0, 3, WebGlRenderingContext::FLOAT, false, 0, 0);
    // dsp.ctx.enable_vertex_attrib_array(0);
    // dsp.ctx.clear_color(0.0, 0.0, 0.0, 1.0);
    // dsp.ctx.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);
    // let num_triangles = (self.vertices.len() / 3) as i32;
    // dsp.ctx.draw_arrays(WebGlRenderingContext::TRIANGLES, 0, num_triangles);

}
