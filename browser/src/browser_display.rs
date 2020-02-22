use nalgebra_glm as glm;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys;
use web_sys::WebGl2RenderingContext as GL;

use crate::types::*;
use regmach::dsp::types as rdt;

impl BrowserDisplay<'_> {
    // todo establish common error handling across project.
    pub fn new() -> BrowserDisplay<'static> {
        log!("init: BrowserDisplay");
        let window = web_sys::window().expect("no global `window` exists");
        let document = window.document().expect("should have a document on window");
        let canvas = document.get_element_by_id("canvas").unwrap();
        let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>().unwrap();
        let context = canvas.get_context("webgl2").unwrap().unwrap();
        let context = context.dyn_into::<GL>().unwrap();
        let wrapper = document.get_element_by_id("canvas_wrapper").unwrap();
        let wrapper = wrapper.dyn_into::<web_sys::HtmlDivElement>().unwrap();

        // TODO figure out how websys exposes the debugging info
        // https://www.khronos.org/webgl/wiki/Debugging

        let mut display = BrowserDisplay { window: window,
                                           canvas: canvas,
                                           wrapper: wrapper,
                                           ctx: context,
                                           events: Rc::new(RefCell::new(vec![])),
                                           props: rdt::DisplayProperties::new(),
                                           camera: Camera::default(),
                                           space_hash: SpaceHash::new(),
                                           font_mgr: FontMgr::new() };

        display.ctx.enable(GL::BLEND);
        display.ctx.blend_func(GL::SRC_ALPHA, GL::ONE_MINUS_SRC_ALPHA);

        display.setup_keydown();
        display.setup_mousedown();
        display.setup_mousemove();

        log!("init: BrowserDisplay succeeds");
        display
    }

    // Command: AddText
    // undo command
    pub fn add_text(&mut self, cmd: rdt::Command) -> Result<rdt::EntityId, String> {
        // add this to entity store

        if let rdt::Command::AddText(x, y, text) = cmd {
            let font = self.font_mgr.font();
            let mut text = Text::new(self, regmach::dsp::colors::JADE_BLUE, &font, &text)?;
            text.move_to(x, y);
            Ok(self.store_entity(box text))
        } else {
            log!("BrowserDisplay::new_text method gets wrong command type: {:?}", cmd);
            Err("BrowserDisplay::new_text method gets wrong command type: {:?}".to_owned())
        }
    }

    pub fn store_entity(&mut self, ent: Box<rdt::Entity>) -> rdt::EntityId {
        self.space_hash.insert(ent)
    }

    pub fn draw_entities(&mut self) {
        // draw all visible entities store
        for ent in self.space_hash.entities_iter() {
            //ent.draw(self);
        }
    }

    fn setup_mousedown(&mut self) {
        let events = self.events.clone();

        let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
                                        log!("mousedown");
                                        let p = rdt::DspPoint { x: event.client_x(), y: event.client_y() };
                                        events.borrow_mut().push(rdt::Event::MouseDown(p));
                                        let msg = format!("events: {:?}", events);
                                        log!("{:?}", msg);
                                    }) as Box<dyn FnMut(_)>);

        let msg = format!("events: {:?}", self.events);
        log!("{:?}", msg);

        let result =
            self.canvas.add_event_listener_with_callback("mousedown", closure.as_ref().unchecked_ref());
        match result {
            Err(msg) => {
                log!("setup_mousedown fails! {:?}", msg);
                panic!("");
            }
            _ => {}
        }

        log!("init: setup_mousedown");
        closure.forget();
    }

    fn setup_mousemove(&mut self) {
        let events = self.events.clone();

        let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
                                        let p = rdt::DspPoint { x: event.client_x(), y: event.client_y() };
                                        events.borrow_mut().push(rdt::Event::MouseMove(p));
                                    }) as Box<dyn FnMut(_)>);

        let msg = format!("events: {:?}", self.events);
        log!("{:?}", msg);

        let result =
            self.canvas.add_event_listener_with_callback("mousemove", closure.as_ref().unchecked_ref());
        match result {
            Err(msg) => {
                log!("setup_mousemove fails! {:?}", msg);
                panic!("");
            }
            _ => {}
        }

        log!("init: setup_mousemove");
        closure.forget();
    }

    fn setup_keydown(&mut self) {
        let events = self.events.clone();

        let closure = Closure::wrap(Box::new(move |event: web_sys::KeyboardEvent| {
                                        log!("keyevent");
                                        let code = event.key_code();
                                        events.borrow_mut().push(rdt::Event::KeyDown(code));
                                    }) as Box<dyn FnMut(_)>);

        let msg = format!("events: {:?}", self.events);
        log!("{:?}", msg);

        let result =
            self.canvas.add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref());
        match result {
            Err(msg) => {
                log!("setup_keydown fails! {:?}", msg);
                panic!("");
            }
            _ => {}
        }

        log!("init: setup_keydown");
        closure.forget();
    }

    // TODO: figure out a better way to update canvas size on window resize
    // right now this checks on every frame in the main loop
    pub fn update_canvas_size_todo(&mut self) {
        let wrapw = self.wrapper.offset_width() as u32;
        let wraph = self.wrapper.offset_height() as u32;
        let canvw = self.canvas.width();
        let canvh = self.canvas.height();

        if wrapw != canvw || wraph != canvh {
            self.canvas.set_width(wrapw);
            self.canvas.set_height(wraph);
            self.camera.update_aspect(wrapw as f64, wraph as f64);
            self.ctx.viewport(0, 0, wrapw as i32, wraph as i32);
        }
    }

    fn height(&self) -> u32 {
        self.wrapper.offset_height() as u32
    }

    fn width(&self) -> u32 {
        self.wrapper.offset_width() as u32
    }

    // generate viewport coordinates from screen coordinates.
    pub fn screen_to_viewport(&self, mouse_x: u32, mouse_y: u32) -> (f32, f32) {
        let w_px = self.width();
        let h_px = self.height();
        let gl_x = 2.0 * (mouse_x as f32 / w_px as f32) - 1.0;
        let gl_y = -2.0 * (mouse_y as f32 / h_px as f32) + 1.0;
        (gl_x, gl_y)
    }

    // cast a ray from the camera into the world down to the schematic grid.
    // maybe there's a better way to do it.
    pub fn screen_to_schematic(&self, mouse_x: u32, mouse_y: u32) -> glm::Vec2 {
        let (vx, vy) = self.screen_to_viewport(mouse_x, mouse_y);
        let w = self.width() as f32;
        let h = self.height() as f32;

        let rat = w / h;
        let c = self.camera.pos.z;
        let a = self.camera.z_near;
        let b = (vx * vx + vy * vy).sqrt();
        let d = c * (b / a);
        let v = glm::vec2(vx, vy).normalize() * d;

        glm::vec2(-v.x * rat - self.camera.pos.x, -v.y + self.camera.pos.y)
    }

    pub fn clear(&self) {
        self.ctx.clear_color(0.98, 0.98, 0.98, 1.0);
        self.ctx.clear(GL::COLOR_BUFFER_BIT);
    }
}

impl<'a> rdt::Display for BrowserDisplay<'_> {
    fn exec(self: &mut Self, cmd: &rdt::Command) {
        // this should push invertible commands onto an undo/redo STACK
    }
    fn exec_cmds(self: &mut Self, cmds: Vec<rdt::Command>) {}
    fn get_events(self: &mut Self) -> Vec<rdt::Event> {
        self.events.borrow_mut().drain(..).collect()
    }
}
