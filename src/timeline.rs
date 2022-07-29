use wasm_bindgen::prelude::*;
use js_sys::*;
use web_sys::{Document, Window};

pub struct TimeLine {
    pub window: Window,
    pub document: Document,
}

impl TimeLine {
    fn create_canvas(&self) {
        let body = self.document.body().expect("document should have a body");

    }
}
