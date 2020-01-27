use crate::types::*;
use regmach::dsp::types as rdt;
use regmach::dsp::types::Display;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::WebGl2RenderingContext;

use rusttype::{point, FontCollection, PositionedGlyph, Scale};
use std::io::Write;

// Called when the wasm module is instantiated
#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    let mut dsp: BrowserDisplay = BrowserDisplay::new();

    let verts: Vec<f32> = vec![-0.7, -0.7, 0.0, 0.7, -0.7, 0.0, 0.0, 0.7, 0.0];
    let mesh = Mesh::from_verts(
        &dsp,
        verts,
        include_str!("../shaders/basic-shader.vs"),
        include_str!("../shaders/basic-shader.fs"),
    )?;

    let grid = Grid::new(&dsp)?;

    // -----------------------------------------------------------------------------
    // this font code is from
    // the rusttype/simple example.

    let font_data = include_bytes!("../../media/font/routed-gothic.ttf");
    let collection = FontCollection::from_bytes(font_data as &[u8]).unwrap_or_else(|e| {
        panic!("error constructing a FontCollection from bytes: {}", e);
    });
    let font = collection
        .into_font() // only succeeds if collection consists of one font
        .unwrap_or_else(|e| {
            panic!("error turning FontCollection into a Font: {}", e);
        });

    // Desired font pixel height
    let height: f32 = 80.4; // to get 80 chars across (fits most terminals); adjust as desired
    let pixel_height = height.ceil() as usize;

    // 2x scale in x direction to counter the aspect ratio of monospace characters.
    let scale = Scale {
        x: height * 1.5,
        y: height * 1.5,
    };

    // The origin of a line of text is at the baseline (roughly where
    // non-descending letters sit). We don't want to clip the text, so we shift
    // it down with an offset when laying it out. v_metrics.ascent is the
    // distance between the baseline and the highest edge of any glyph in
    // the font. That's enough to guarantee that there's no clipping.
    let v_metrics = font.v_metrics(scale);
    let offset = point(0.0, v_metrics.ascent);

    // Glyphs to draw for "RustType". Feel free to try other strings.
    let glyphs: Vec<PositionedGlyph<'_>> = font.layout("MEM[31:0]", scale, offset).collect();

    // Find the most visually pleasing width to display
    let width = glyphs
        .iter()
        .rev()
        .map(|g| g.position().x as f32 + g.unpositioned().h_metrics().advance_width)
        .next()
        .unwrap_or(0.0)
        .ceil() as usize;
    
    let mut text_verts: Vec<f32> = vec![];
    let mut colors: Vec<f32> = vec![];
    let (red,green, blue, _) = regmach::dsp::colors::JADE_BLUE.as_gl();

    
    for glyph in glyphs.iter().rev() {
        // this clone is just to appease the rust type checker, it will be going away.
        if let Some(bb) = glyph.pixel_bounding_box() {
            // let mpx = (bb.max.x - bb.min.x) as f32;
            // let mpy = (bb.max.y - bb.min.y) as f32;
            
            glyph.draw(|x, y, v| {
                if v > 0.3 {
                    // v should be in the range 0.0 to 1.0
                    let scale = 0.01;
                    let x = -((x as i32 + bb.min.x) as f32 * scale);
                    let y = -((y as i32 + bb.min.y) as f32 * scale);

                    // need to draw two small triangles per pixel.
                    // or use different mesh routines with four points using gl_fan.
                    // but for now 6 points per font pixel.
                    // what coordinate system is being used?
                    let bl = [x, y, 0.0];
                    let br = [x + scale, y, 0.0];
                    let tl = [x, y + scale, 0.0];
                    let tr = [x + scale, y + scale, 0.0];

                    
                    let co = [red, green, blue, v];

                    // triangle 1: counter clockwise: bl br tl
                    text_verts.extend(bl.iter());
                    colors.extend(co.iter());
                    text_verts.extend(br.iter());
                    colors.extend(co.iter());
                    text_verts.extend(tl.iter());
                    colors.extend(co.iter()); 
                   
                    //triangle 2: counter clockwise: tl br tr
                    text_verts.extend(tl.iter());
                    colors.extend(co.iter());
                    text_verts.extend(br.iter());
                    colors.extend(co.iter());
                    text_verts.extend(tr.iter());
                    colors.extend(co.iter());
                }
            })
        }
    }
     
    let text_mesh = FontMesh::from_verts(
        &dsp,
        text_verts,
        colors,
        include_str!("../shaders/font-shader.vs"),
        include_str!("../shaders/font-shader.fs"),
    )?;

    // -----------------------------------------------------------------------------
    // MAIN EVENT LOOP
    // https://rustwasm.github.io/wasm-bindgen/examples/request-animation-frame.html

    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        dsp.update_canvas_size_todo();
        dsp.props.frame_increment();
        dsp.ctx.clear_color(0.98, 0.98, 0.98, 1.0);
        dsp.ctx.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);
        for ev in &dsp.get_events() {
            // there should be an event driven way adjust the canvas size.
            // dsp.watch_for_window_resize_awful();
            //
            match ev {
                rdt::Event::MouseDown(p) => {
                    log!("processing {:?}, vertex_buffer: {:?}", ev, mesh.vertex_buffer);
                    let scmpoint = dsp.screen_to_schematic(p.x as u32, p.y as u32);
                    log!("scmpoint: {:?}", scmpoint);
                }
                rdt::Event::MouseMove(p) => {
                    //log!("processing {:?}", ev);
                }
                rdt::Event::KeyDown(code) => {
                    log!("processing {:?}", ev);
                    match *code {
                        65 => dsp.camera.pan_left(),
                        68 => dsp.camera.pan_right(),
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
        mesh.draw_with_mode(&dsp, WebGl2RenderingContext::TRIANGLES);
        text_mesh.draw_with_mode(&dsp, WebGl2RenderingContext::TRIANGLES);

        // ------------------------------------------------------------------
        // font rendering
        /*
         */
        // ------------------------------------------------------------------

        // Schedule another requestAnimationFrame callback.
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
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

#[wasm_bindgen]
pub fn add(a: u32, b: u32) -> u32 {
    a + b
}
