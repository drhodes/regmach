// use nalgebra;
use nalgebra_glm as glm;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys;
use web_sys::WebGl2RenderingContext;

use crate::types::*;
use regmach::dsp::types as rdt;

impl BrowserDisplay {
    // todo establish common error handling across project.
    pub fn new() -> BrowserDisplay {
        log!("init: BrowserDisplay");
        let window = web_sys::window().expect("no global `window` exists");
        let document = window.document().expect("should have a document on window");
        let canvas = document.get_element_by_id("canvas").unwrap();
        let canvas: web_sys::HtmlCanvasElement =
            canvas.dyn_into::<web_sys::HtmlCanvasElement>().unwrap();
        let context = canvas.get_context("webgl2").unwrap().unwrap();
        let context = context.dyn_into::<WebGl2RenderingContext>().unwrap();
        let wrapper = document.get_element_by_id("canvas_wrapper").unwrap();
        let wrapper = wrapper.dyn_into::<web_sys::HtmlDivElement>().unwrap();
        log!("..init: got webgl2 context from browser");

        // TODO figure out how websys exposes the debugging info
        // https://www.khronos.org/webgl/wiki/Debugging

        let mut display = BrowserDisplay {
            window: window,
            canvas: canvas,
            wrapper: wrapper,
            ctx: context,
            events: Rc::new(RefCell::new(vec![])),
            props: rdt::DisplayProperties::new(),
            camera: Camera::default(),
            mesh_nonce: 0,
            mesh_store: HashMap::new(),
        };

        display.setup_keydown();
        display.setup_mousedown();
        display.setup_mousemove();

        log!("init: BrowserDisplay succeeds");
        display
    }

    fn setup_mousedown(&mut self) {
        let events = self.events.clone();

        let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            log!("mousedown");
            let p = rdt::DspPoint {
                x: event.client_x(),
                y: event.client_y(),
            };
            events.borrow_mut().push(rdt::Event::MouseDown(p));
            let msg = format!("events: {:?}", events);
            log!("{:?}", msg);
        }) as Box<dyn FnMut(_)>);

        let msg = format!("events: {:?}", self.events);
        log!("{:?}", msg);

        let result = self
            .canvas
            .add_event_listener_with_callback("mousedown", closure.as_ref().unchecked_ref());
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
            //log!("mousemove");
            let p = rdt::DspPoint {
                x: event.client_x(),
                y: event.client_y(),
            };
            events.borrow_mut().push(rdt::Event::MouseMove(p));
            //let msg = format!("events: {:?}", events);
            //log!("{:?}", msg);
        }) as Box<dyn FnMut(_)>);

        let msg = format!("events: {:?}", self.events);
        log!("{:?}", msg);

        let result = self
            .canvas
            .add_event_listener_with_callback("mousemove", closure.as_ref().unchecked_ref());
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

        let result = self
            .canvas
            .add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref());
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
        let mut w: f64 = 0.0;
        let mut h: f64 = 0.0;
        if self.window.inner_width().unwrap() != self.canvas.width()
            || self.window.inner_height().unwrap() != self.canvas.height()
        {
            w = self.window.inner_width().unwrap().as_f64().unwrap();
            h = self.window.inner_height().unwrap().as_f64().unwrap();
            self.canvas.set_width(w.floor() as u32);
            self.canvas.set_height(h.floor() as u32);
            self.camera.update_aspect(w, h);
            self.ctx.viewport(0, 0, w as i32, h as i32);
        }
    }

    fn height(&self) -> Option<u32> {
        self.window
            .inner_height()
            .expect("fails to get inner width on window")
            .as_f64()
            .map(|y| y as u32)
    }

    fn width(&self) -> Option<u32> {
        self.window
            .inner_width()
            .expect("fails to get inner width on window")
            .as_f64()
            .map(|x| x as u32)
    }

    // generate viewport coordinates from screen coordinates.
    pub fn screen_to_viewport(&self, mouse_x: u32, mouse_y: u32) -> (f32, f32) {
        let w_px = self.width().unwrap();
        let h_px = self.height().unwrap();
        let gl_x = 2.0 * (mouse_x as f32 / w_px as f32) - 1.0;
        let gl_y = -2.0 * (mouse_y as f32 / h_px as f32) + 1.0;
        (gl_x, gl_y)
    }

    // The joys of mouse picking.
    // https://stackoverflow.com/questions/29997209/opengl-c-mouse-ray-picking-glmunproject
    fn screen_to_world_ray(&self, mouse_x: u32, mouse_y: u32) -> glm::Vec4 {
        let (vp_x, vp_y) = self.screen_to_viewport(mouse_x, mouse_y);
        let view = self.camera.view_matrix();
        let proj = self.camera.projection_matrix();
        let mouse = glm::vec4(vp_x, -vp_y, 1.0, 1.0);
        let p = glm::inverse(&(proj * view)) * mouse;
        p
    }

    // cast a ray from the camera into the world down to the schematic grid.
    // maybe there's a better way to do it.
    pub fn screen_to_schematic(&self, mouse_x: u32, mouse_y: u32) -> glm::Vec2 {
        let ray = self.screen_to_world_ray(mouse_x, mouse_y);
        let ray = glm::normalize(&ray);
        let zhat = glm::vec4(0.0, 0.0, -1.0, 0.0);
        let dotp = ray.dot(&zhat);
        let mag = self.camera.pos.z.abs() / dotp;
        let shift = glm::vec4(self.camera.pos.x, self.camera.pos.y, 0.0, 0.0);
        let worldp = ray * mag - shift;
        glm::vec2(worldp.x, worldp.y)
    }
}

impl<'a> rdt::Display for BrowserDisplay {
    fn exec(self: &mut Self, cmd: &rdt::Command) {}
    fn exec_cmds(self: &mut Self, cmds: Vec<rdt::Command>) {}
    fn get_events(self: &mut Self) -> Vec<rdt::Event> {
        self.events.borrow_mut().drain(..).collect()
    }
}
