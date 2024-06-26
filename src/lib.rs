use std::{cell::RefCell, rc::Rc};

use canvas::{rect, Canvas};
use wasm_bindgen::prelude::*;
use web_sys::console;

mod canvas;

fn update() {
    // console::log_1(&JsValue::from_str("update"));
}

fn render() {
    update();
    rect(0, 0, 300, 160, &canvas::Color::new(229, 37, 37, 255));
}

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn set_up() -> Result<(), JsValue> {
    Canvas::new(
        Rc::new(RefCell::new(render)),
        Rc::new(RefCell::new(key_down)),
        Rc::new(RefCell::new(key_up)),
    )
    .run();

    Ok(())
}

fn key_down(key: &str) {
    console::log_1(&JsValue::from_str(format!("key_down: {}", key).as_str()));
}

fn key_up(key: &str) {
    console::log_1(&JsValue::from_str(format!("key_up: {}", key).as_str()));
}
