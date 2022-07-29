use std::f64;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
// use js_sys::*;
use web_sys::{CanvasRenderingContext2d, Document, Element, HtmlCanvasElement, HtmlDivElement, HtmlElement, Window};

pub struct TimeLine {
    pub window: Window,
    pub document: Document,
    pub element_id: String,
    canvas: HtmlCanvasElement,
}

impl TimeLine {
    pub fn new(window: Window, document: Document, element_id: String) -> Self {
        let body = document.body().expect("document should have a body");
        // Create the canvas element
        let wrapper_el = document.get_element_by_id("webgl-timeline").unwrap();
        let canvas_el = document.create_element("canvas").expect("should create canvas element");
        canvas_el.set_attribute("id", "__webgl_timelines__").unwrap();
        wrapper_el.append_with_node_1(&canvas_el).expect("should append node");
        body.append_child(&wrapper_el).expect("sm");
        // canvas
        let canvas = document.get_element_by_id("__webgl_timelines__").unwrap();
        let canvas: HtmlCanvasElement = canvas
            .dyn_into::<HtmlCanvasElement>()
            .map_err(|_| ())
            .unwrap();

        TimeLine{
            window,
            document,
            element_id,
            canvas,
        }
    }

    pub fn create_ctx(&self) -> CanvasRenderingContext2d {
        let context = self.canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()
            .unwrap();
        context
    }

    pub fn draw_line(&self, ctx: &CanvasRenderingContext2d) {
        ctx.begin_path();
        ctx.arc(75.0, 75.0, 50.0, 0.0, f64::consts::PI * 2.0)
            .unwrap();
        ctx.stroke();
    }
}
