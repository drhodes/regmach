use wasm_bindgen::prelude::*;

// https://rustwasm.github.io/docs/wasm-bindgen/reference/types/boxed-jsvalue-slice.html
#[wasm_bindgen(module = "js/event-queue.js")]
extern "C" {
    #[wasm_bindgen]
    pub fn pop_event() -> String;

// #[wasm_bindgen(method)]
// fn render(this: &MyClass) -> String;
}
