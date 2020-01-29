use crate::gl_util;
use crate::types::*;
use crate::types::*;
use regmach::dsp::types as rdt;
use regmach::dsp::types::Display;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys;
use web_sys::{WebGl2RenderingContext, WebGlBuffer};

use rusttype::{point, FontCollection, PositionedGlyph, Scale};
use std::io::Write;

impl Text {
    pub fn new(dsp: &BrowserDisplay,
               color: rdt::Color,
               font: &rusttype::Font<'_>,
               text: &str)
               -> Result<Text, String> {
        // Desired font pixel height
        let height: f32 = 80.4;
        let pixel_height = height.ceil() as usize;

        let scale = Scale { x: height * 1.5, y: height * 1.5 };

        // The origin of a line of text is at the baseline (roughly where
        // non-descending letters sit). We don't want to clip the text, so we shift
        // it down with an offset when laying it out. v_metrics.ascent is the
        // distance between the baseline and the highest edge of any glyph in
        // the font. That's enough to guarantee that there's no clipping.
        let v_metrics = font.v_metrics(scale);
        let offset = point(0.0, v_metrics.ascent);

        // Glyphs to draw for "RustType". Feel free to try other strings.
        let glyphs: Vec<PositionedGlyph<'_>> = font.layout(text, scale, offset).collect();

        // Find the most visually pleasing width to display
        let width =
            glyphs.iter()
                  .rev()
                  .map(|g| g.position().x as f32 + g.unpositioned().h_metrics().advance_width)
                  .next()
                  .unwrap_or(0.0)
                  .ceil() as usize;

        let mut text_verts: Vec<f32> = vec![];
        let mut colors: Vec<f32> = vec![];
        let (red, green, blue, _) = color.as_gl();

        for glyph in glyphs.iter().rev() {
            // this clone is just to appease the rust type checker, it will be going away.
            if let Some(bb) = glyph.pixel_bounding_box() {
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

                             // this is inefficient, TODO (just send one
                             // color per triangle)

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

        let font_mesh = FontMesh::from_verts(&dsp,
                                             text_verts,
                                             colors,
                                             include_str!("../shaders/font-shader.vs"),
                                             include_str!("../shaders/font-shader.fs"))?;

        Ok(Text { color, font_mesh, text: text.to_owned() })
    }

    /// Mode is LINES, TRIANGLE, TRIANGLESTRIP, etc.
    pub fn draw_with_mode(&self, dsp: &BrowserDisplay, mode: u32) {
        self.font_mesh.draw_with_mode(dsp, mode);
    }
    pub fn move_to(&mut self, x: f32, y: f32) {
        self.font_mesh.move_to(x, y);
    }
}
