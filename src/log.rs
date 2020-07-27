use wasm_bindgen::prelude::*;
use web_sys::console;
use crate::Context;

pub fn log(_context: &mut Context, message: impl Into<JsValue>) {
    // TODO rust analyzer seems to get confused here about wherether unsafe is required or not
    unsafe {
        console::log_1(&(message.into()));
    }
}
