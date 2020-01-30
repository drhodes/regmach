use crate::types::*;
use regmach::dsp::types as rdt;
use regmach::dsp::types::Display;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::WebGl2RenderingContext as GL;

use rusttype::{point, FontCollection, PositionedGlyph, Scale};
use std::io::Write;

// Called when the wasm module is instantiated
#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    let mut dsp: BrowserDisplay = BrowserDisplay::new();

    let verts: Vec<f32> = vec![-0.7, -0.7, 0.0, 0.7, -0.7, 0.0, 0.0, 0.7, 0.0];
    let mut triangle = Mesh::from_verts(&dsp,
                                        verts,
                                        include_str!("../shaders/basic-shader.vs"),
                                        include_str!("../shaders/basic-shader.fs"))?;

    let grid = Grid::new(&dsp)?;
    let mut texts = vec![];
    for i in (0..=100).step_by(5) {
        texts.push(dsp.add_text(rdt::Command::AddText(i as f32 + 0.0,
                                                      i as f32 + 1.0,
                                                      format!("({:?}, {:?})", i, i).to_owned()))?);
    }

    // -----------------------------------------------------------------------------
    // MAIN EVENT LOOP
    // https://rustwasm.github.io/wasm-bindgen/examples/request-animation-frame.html

    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    *g.borrow_mut() =
        Some(Closure::wrap(Box::new(move || {
                               // there should be an event driven way adjust the canvas size.
                               dsp.update_canvas_size_todo();
                               dsp.props.frame_increment();
                               dsp.clear();

                               for ev in &dsp.get_events() {
                                   // there should be an event driven way adjust the canvas size.
                                   match ev {
                                       rdt::Event::MouseDown(p) => {
                                           log!("processing {:?}, vertex_buffer: {:?}",
                                                ev,
                                                triangle.vertex_buffer);
                                           let scmpoint = dsp.screen_to_schematic(p.x as u32, p.y as u32);
                                           log!("scmpoint: {:?}", scmpoint);
                                       }
                                       rdt::Event::MouseMove(p) => {
                                           let pos = dsp.screen_to_schematic(p.x as u32, p.y as u32);
                                           // triangle.move_to(pos.x, pos.y);
                                       }
                                       rdt::Event::KeyDown(code) => {
                                           log!("processing {:?}", ev);
                                           match *code {
                                               68 => dsp.camera.pan_left(),
                                               65 => dsp.camera.pan_right(),
                                               87 => dsp.camera.pan_up(),
                                               83 => dsp.camera.pan_down(),
                                               33 => dsp.camera.zoom_in(),
                                               34 => dsp.camera.zoom_out(),
                                               67 => dsp.camera.center(),
                                               _ => log!("unhandled key {:?}", ev),
                                           }
                                       }
                                       _ => {
                                           log!("unhandled event: {:?}", ev);
                                       }
                                   }
                               }

                               grid.draw(&dsp);
                               triangle.draw_with_mode(&dsp, GL::TRIANGLES);

                               for text in texts.iter() {
                                   text.draw_with_mode(&dsp, GL::TRIANGLES);
                               }

                               // schedule another requestAnimationFrame callback.
                               request_animation_frame(f.borrow().as_ref().unwrap());
                           }) as Box<dyn FnMut()>));

    request_animation_frame(g.borrow().as_ref().unwrap());
    Ok(())
}

fn document() -> web_sys::Document {
    window().document().expect("should have a document on window")
}

fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window().request_animation_frame(f.as_ref().unchecked_ref())
            .expect("should register `requestAnimationFrame` OK");
}

#[wasm_bindgen]
pub fn add(a: u32, b: u32) -> u32 {
    a + b
}
