use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::WebGl2RenderingContext;
use web_sys;
use std::collections::HashMap;

use crate::types::*;
use regmach::dsp::types as rdt;

impl BrowserDisplay {
    // todo establish common error handling across project.
    pub fn new() -> BrowserDisplay {
        log!("init: BrowserDisplay");

        let window = web_sys::window().expect("no global `window` exists");
        let document = window.document().expect("should have a document on window");
        let canvas = document.get_element_by_id("canvas").unwrap();
        let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>().unwrap();
        
        let context = canvas.get_context("webgl2").unwrap().unwrap();
        let context = context.dyn_into::<WebGl2RenderingContext>().unwrap();
        log!("..init: got webgl2 context from browser");

        // TODO figure out how websys exposes the debugging info
        // https://www.khronos.org/webgl/wiki/Debugging
        
        let mut display = BrowserDisplay {
            canvas: canvas,
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

    fn setup_keydown(&mut self) {
        let mut events = self.events.clone();

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


    fn setup_onresize(&mut self) {
        let mut events = self.events.clone();

        let closure = Closure::wrap(Box::new(move |event: web_sys::Window| {
            log!("onresize");
            // let code = event.key_code();
            // events.borrow_mut().push(rdt::Event::Onresize(code));
        }) as Box<dyn FnMut(_)>);

        let msg = format!("events: {:?}", self.events);
        log!("{:?}", msg);

        let result = self
            .canvas
            .add_event_listener_with_callback("onresize", closure.as_ref().unchecked_ref());
        match result {
            Err(msg) => {
                log!("setup_onresize fails! {:?}", msg);
                panic!("");
            }
            _ => {}
        }

        log!("init: setup_onresize");
        closure.forget();
    }

    
    
}

impl<'a> rdt::Display for BrowserDisplay {
    fn exec(self: &mut Self, cmd: &rdt::Command) {}
    fn exec_cmds(self: &mut Self, cmds: Vec<rdt::Command>) {}
    fn get_events(self: &mut Self) -> Vec<rdt::Event> {
        self.events.borrow_mut().drain(..).collect()
    }
}
