use crate::Closure;
use crate::JsValue;
use once_cell::sync::Lazy;
use std::ops::DerefMut;
use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::Clamped;
use wasm_bindgen::JsCast;
use web_sys::CanvasRenderingContext2d;
use web_sys::ImageData;

type DrawFn = Rc<RefCell<dyn FnMut() + 'static>>;

pub struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Color { r, g, b, a }
    }
}

pub type KeyBoard = Rc<RefCell<dyn FnMut(&str) + 'static>>;

pub static mut CANVAS: Lazy<Option<web_sys::HtmlCanvasElement>> = Lazy::new(|| None);

pub fn create_offset_canvas() {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.create_element("canvas").unwrap();
    let canvas = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .ok();

    if let Some(ref canvas) = canvas {
        canvas.set_width(600);
        canvas.set_height(800);
    }

    unsafe {
        *CANVAS = canvas;
    }
}

pub fn get_content() -> CanvasRenderingContext2d {
    unsafe {
        let context_options = web_sys::js_sys::Object::new();
        web_sys::js_sys::Reflect::set(
            &context_options,
            &JsValue::from_str("willReadFrequently"),
            &JsValue::from_bool(true),
        )
        .unwrap();

        let context = CANVAS
            .clone()
            .unwrap()
            .get_context_with_context_options("2d", &context_options)
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .ok();

        context.unwrap()
    }
}

pub struct Canvas {
    pub draw: DrawFn,
    pub key_down: KeyBoard,
    pub key_up: KeyBoard,
}

impl Canvas {
    pub fn new(draw: DrawFn, key_down: KeyBoard, key_up: KeyBoard) -> Self {
        Canvas {
            // context: None,
            draw: draw,
            key_down: key_down,
            key_up: key_up,
        }
    }

    pub fn run(&self) {
        create_offset_canvas();
        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document.get_element_by_id("canvas").unwrap();
        let canvas: web_sys::HtmlCanvasElement = canvas
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .map_err(|_| ())
            .unwrap();

        let context = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();

        let f = Rc::new(RefCell::new(None));
        let g = f.clone();

        let draw = self.draw.clone();
        let key_down = self.key_down.clone();
        let key_up = self.key_up.clone();

        let keydown = Closure::<dyn FnMut(_)>::new(move |event: web_sys::KeyboardEvent| {
            let key = event.key().to_uppercase();
            let key = key.as_str();
            key_down.borrow_mut()(key);
        });

        let keyup = Closure::<dyn FnMut(_)>::new(move |event: web_sys::KeyboardEvent| {
            let key = event.key().to_uppercase();
            let key = key.as_str();
            key_up.borrow_mut()(key);
        });

        document
            .add_event_listener_with_callback("keydown", keydown.as_ref().unchecked_ref())
            .unwrap();

        document
            .add_event_listener_with_callback("keyup", keyup.as_ref().unchecked_ref())
            .unwrap();

        keyup.forget();
        keydown.forget();

        *g.borrow_mut() = Some(Closure::new(move || {
            // Set the body's text content to how many times this
            draw.borrow_mut()();

            unsafe {
                context
                    .draw_image_with_html_canvas_element(CANVAS.as_ref().unwrap(), 0.0, 0.0)
                    .unwrap();
            }
            // Schedule ourself for another requestAnimationFrame callback.
            request_animation_frame(f.borrow().as_ref().unwrap());
        }));

        request_animation_frame(g.borrow().as_ref().unwrap());
    }
}

pub fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

pub fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

pub fn get_image_data() -> web_sys::ImageData {
    let ctx = get_content();
    let image_data = ctx.get_image_data(0.0, 0.0, 600.0, 800.0).unwrap();
    image_data
}

pub fn set_image_data(data: Vec<u8>) {
    let ctx = get_content();
    let image_data = ImageData::new_with_u8_clamped_array_and_sh(Clamped(&data), 600, 800).unwrap();
    ctx.put_image_data(&image_data, 0.0, 0.0).unwrap();
}

pub fn clear() {
    let image_data = get_image_data();
    let mut data = image_data.data();
    for i in (0..data.len()).step_by(4) {
        data[i] = 0;
        data[i + 1] = 0;
        data[i + 2] = 0;
        data[i + 3] = 0;
    }
    // set_image_data(data);
}

pub fn rect(x: usize, y: usize, w: usize, h: usize, color: &Color) {
    let image_data = get_image_data();
    let mut data = image_data.data();
    let data = data.deref_mut();
    for i in (0..h).step_by(4) {
        for j in (0..w).step_by(4) {
            draw_pixel_by_vec(x + j, y + i, color, data);
        }
    }

    set_image_data(data.clone());
}

pub fn draw_pixel_by_vec(x: usize, y: usize, color: &Color, data: &mut Vec<u8>) {
    let i: usize = y * 600 + x;
    data[i] = color.r;
    data[i + 1] = color.g;
    data[i + 2] = color.b;
    data[i + 3] = color.a;
}
