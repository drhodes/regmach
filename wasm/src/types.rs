use regmach::dsp::types as rdt;
use std::cell::RefCell;
use std::rc::Rc;
use web_sys::WebGlRenderingContext;

pub struct WasmDisplay {
    pub canvas: web_sys::HtmlCanvasElement,
    pub ctx: WebGlRenderingContext,
    pub events: Rc<RefCell<Vec<rdt::Event>>>,
    pub props: rdt::DisplayProperties,
}
