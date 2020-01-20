use crate::types::*;
use regmach::dsp::types as rdt;
use regmach::dsp::types::Display;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

// Called when the wasm module is instantiated
#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {    
    let mut dsp: BrowserDisplay = BrowserDisplay::new();
    
    let verts: Vec<f32> = vec!(-0.7, -0.7, 0.0, 0.7, -0.7, 0.0, 0.0, 0.7, 0.0);
    let mesh = Mesh::from_verts(&dsp, verts)?; 
    
    // -----------------------------------------------------------------------------
    // MAIN EVENT LOOP
    // https://rustwasm.github.io/wasm-bindgen/examples/request-animation-frame.html

    let f = Rc::new(RefCell::new(None));
    let g = f.clone();
    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        dsp.props.frame_increment();
        if dsp.props.frame % 10 == 0 {
            dsp.camera.zoom_out();
            dsp.camera.move_left();
        }

            
        for ev in &dsp.get_events() {
            match ev {
                rdt::Event::MouseDown(p) => {
                    log!("processing {:?}, vertex_buffer: {:?}", ev, mesh.vertex_buffer);
                },
                rdt::Event::MouseMove(p) => {
                    log!("processing {:?}", ev);
                },
                _ => {
                    log!("unhandled event: {:?}", ev);
                }
            }
        }
        mesh.draw(&dsp);
        // Schedule another requestAnimationFrame callback.
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
