mod chip_eight;
mod utils;

use js_sys::Array;
use wasm_bindgen::{closure::Closure, prelude::*, JsCast};

use chip_eight::ChipEight;

use std::cell::RefCell;
use std::rc::Rc;

#[wasm_bindgen]
pub struct WasmChipEight {
    chip_eight: ChipEight,
}

#[wasm_bindgen]
impl WasmChipEight {
    #[wasm_bindgen(constructor)]
    pub fn new(memory: &[u8]) -> Self {
        assert!(memory.len() == chip_eight::MEMORY_SIZE);

        Self {
            chip_eight: ChipEight::new(memory),
        }
    }

    pub fn step(&mut self) -> u16 {
        self.chip_eight.step()
    }

    pub fn get_memory(&self, idx: usize) -> u8 {
        self.chip_eight.get_memory(idx)
    }
}

fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

fn document() -> web_sys::Document {
    window()
        .document()
        .expect("should have a document on window")
}

fn body() -> web_sys::HtmlElement {
    document().body().expect("document should have a body")
}

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    utils::set_panic_hook();
    let window = window();
    let document = document();

    let canvas_element = document
        .get_element_by_id("chip-8-canvas")
        .ok_or("canvas not found")?;

    let canvas: web_sys::HtmlCanvasElement = canvas_element
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| "canvas is not valid HtmlCanvasElement")?;

    let width = 500;
    let height = 500;

    canvas.set_width(width);
    canvas.set_height(height);

    canvas
        .style()
        .set_property("border", "solid")
        .map_err(|_| "error setting solid border")?;

    let context = canvas
        .get_context("2d")
        .map_err(|_| "error getting context")?
        .ok_or("error getting 2d context")?
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .map_err(|_| "2d context isn't valid CanvasRenderingContext2d")?;

    context.set_fill_style(&JsValue::from("blue"));

    let callback = Rc::new(RefCell::new(None));
    let callback_move = Rc::clone(&callback);

    let mut dx = 3.0;
    let mut dy = 4.0;
    let mut x = 80.0;
    let mut y = 80.0;
    let box_width = 50.0;
    let box_height = 50.0;

    *callback.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        context.clear_rect(0.0, 0.0, canvas.width().into(), canvas.height().into());
        context.fill_rect(x, y, 50.0, 50.0);

        if x + dx + box_width >= width.into() || x + dx < 0.0 {
            dx *= -1.0;
        }

        if y + dy + box_height >= height.into() || y + dx < 0.0 {
            dy *= -1.0;
        }

        x += dx;
        y += dy;

        request_animation_frame(callback_move.clone().borrow().as_ref().unwrap());
    }) as Box<dyn FnMut()>));

    request_animation_frame(callback.borrow().as_ref().unwrap());

    Ok(())
}
