use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::WebGlRenderingContext;

use crate::gl_util;
use crate::types::*;
use regmach::dsp::types as rdt;
use regmach::dsp::types::Display;

impl BrowserDisplay {
    // todo establish common error handling across project.
    pub fn new() -> BrowserDisplay {
        let window = web_sys::window().expect("no global `window` exists");
        let document = window.document().expect("should have a document on window");
        let canvas = document.get_element_by_id("canvas").unwrap();
        let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>().unwrap();

        let context = canvas
            .get_context("webgl")
            .unwrap()
            .unwrap()
            .dyn_into::<WebGlRenderingContext>()
            .unwrap();

        let mut display = BrowserDisplay {
            canvas: canvas,
            ctx: context,
            events: Rc::new(RefCell::new(vec![])),
            props: rdt::DisplayProperties::new(),
        };
        display.setup_mousedown();
        display.setup_mousemove();
        log!("init: BrowserDisplay");
        display
    }
    
    fn setup_mousedown(&mut self) {
        let mut events = self.events.clone();

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

        let result = self.canvas.add_event_listener_with_callback("mousedown", closure.as_ref().unchecked_ref());
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
        let mut events = self.events.clone();

        let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            log!("mousemove");
            let p = rdt::DspPoint {
                x: event.client_x(),
                y: event.client_y(),
            };
            events.borrow_mut().push(rdt::Event::MouseMove(p));
            let msg = format!("events: {:?}", events);
            log!("{:?}", msg);
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

    pub fn load_vertex_shader(&self) -> Result<web_sys::WebGlShader, String> {
        gl_util::compile_shader(
            &self.ctx,
            WebGlRenderingContext::VERTEX_SHADER,
            r#"
            attribute vec4 position;
            void main() {
            gl_Position = position;
            }
            "#,
        )
    }

    pub fn load_fragment_shader(&self) -> Result<web_sys::WebGlShader, String> {
        gl_util::compile_shader(
            &self.ctx,
            WebGlRenderingContext::FRAGMENT_SHADER,
            r#"
            void main() {
                gl_FragColor = vec4(1.0, 0.0, 1.0, 1.0);
            }
             "#,
        ) 
    }

    
    
    
}

impl<'a> rdt::Display for BrowserDisplay {
    fn exec(self: &mut Self, cmd: &rdt::Command) {}
    fn exec_cmds(self: &mut Self, cmds: Vec<rdt::Command>) {}
    fn get_events(self: &mut Self) -> Vec<rdt::Event> {
        self.events.borrow_mut().drain(..).collect()
    }
}
