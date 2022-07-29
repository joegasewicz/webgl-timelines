mod timeline;

use wasm_bindgen::prelude::*;
use js_sys::*;
use crate::timeline::TimeLine;

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(_name: &str) {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have document on window");
    let timeline =  TimeLine {
        window,
        document,
    };
    //
}
