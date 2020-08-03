use wasm_bindgen::prelude::*;
use web_sys::console;
use crate::Context;

// TODO rust analyzer seems to get confused here about wherether unsafe is required or not
#[allow(unused_unsafe)]
pub fn log(_context: &mut Context, message: impl Into<JsValue>) {
    unsafe {
        console::log_1(&(message.into()));
    }
}
