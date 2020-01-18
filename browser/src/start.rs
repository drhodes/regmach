use crate::gl_util;
use crate::types::*;
use regmach::dsp::types as rdt;
use regmach::dsp::types::Display;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{WebGlBuffer, WebGlRenderingContext};

// Called when the wasm module is instantiated
#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    let mut dsp: BrowserDisplay = BrowserDisplay::new();
    
    let vert_shader = dsp.load_vertex_shader()?;
    let frag_shader = dsp.load_fragment_shader()?;
    let program = gl_util::link_program(&dsp.ctx, &vert_shader, &frag_shader)?;
    dsp.ctx.use_program(Some(&program));
    let vertices: [f32; 9] = [-0.7, -0.7, 0.0, 0.7, -0.7, 0.0, 0.0, 0.7, 0.0];
    let buffer: WebGlBuffer = dsp.ctx.create_buffer().ok_or("failed to create buffer")?;
    dsp.ctx.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&buffer));

    unsafe {
        // Note that `Float32Array::view` is somewhat dangerous (hence the
        // `unsafe`!). This is creating a raw view into our module's
        // `WebAssembly.Memory` buffer, but if we allocate more pages for ourdsp
        // (aka do a memory allocation in Rust) it'll cause the buffer to change,
        // causing the `Float32Array` to be invalid.
        //
        // As a result, after `Float32Array::view` we have to be very careful not to
        // do any memory allocations before it's dropped.
        let vert_array = js_sys::Float32Array::view(&vertices);
        dsp.ctx.buffer_data_with_array_buffer_view(
            WebGlRenderingContext::ARRAY_BUFFER,
            &vert_array,
            WebGlRenderingContext::STATIC_DRAW,
        );
    }

    dsp.ctx.vertex_attrib_pointer_with_i32(0, 3, WebGlRenderingContext::FLOAT, false, 0, 0);
    dsp.ctx.enable_vertex_attrib_array(0);
    dsp.ctx.clear_color(0.0, 0.0, 0.0, 1.0);
    dsp.ctx.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);
    let num_triangles = (vertices.len() / 3) as i32;
    dsp.ctx.draw_arrays(WebGlRenderingContext::TRIANGLES, 0, num_triangles);

    
    // ------------------------------------------------------------------
    // MAIN EVENT LOOP
    // https://rustwasm.github.io/wasm-bindgen/examples/request-animation-frame.html

    let f = Rc::new(RefCell::new(None));
    let g = f.clone();
    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        for ev in &dsp.get_events() {
            match ev {
                rdt::Event::MouseDown(p) => log!("processing {:?}", ev),
                rdt::Event::MouseMove(p) => log!("processing {:?}", ev),
                _ => {
                    log!("unhandled event: {:?}", ev);
                }
            }
        }

        dsp.ctx.clear_color(0.95, 0.95, 0.95, 1.0);
        dsp.ctx.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);
        dsp.ctx.draw_arrays(
            WebGlRenderingContext::TRIANGLES,
            0,
            (vertices.len() / 3) as i32,
        );
        // Schedule ourdsp for another requestAnimationFrame callback.
        request_animation_frame(f.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut()>));
    
    request_animation_frame(g.borrow().as_ref().unwrap());
    Ok(())
        
}

fn document() -> web_sys::Document {
    window()
        .document()
        .expect("should have a document on window")
}

fn body() -> web_sys::HtmlElement {
    document().body().expect("document should have a body")
}

fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

#[wasm_bindgen]
pub fn add(a: u32, b: u32) -> u32 {
    a + b
}
