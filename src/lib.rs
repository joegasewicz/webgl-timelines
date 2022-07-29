mod timeline;

use wasm_bindgen::prelude::*;
use js_sys::*;
use crate::timeline::TimeLine;

#[wasm_bindgen]
pub fn newTimeline(_name: &str) {
    let element_id = String::from("webgl-timeline");
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have document on window");
    let timeline =  TimeLine::new(window, document, element_id);
}
