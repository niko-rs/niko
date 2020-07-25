mod context;
mod error;
mod game;

pub use context::*;
pub use error::*;
pub use game::*;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::WebGlRenderingContext;
use std::cell::RefCell;
use std::rc::Rc;
use std::panic;

macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

pub fn clear(context: &mut Context, r: f32, g: f32, b: f32, a: f32) {
    context.rendering_context.clear_color(r, g, b, a);
    context.rendering_context.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);
}

pub fn run(mut game: Box<dyn Game>) -> Result<(), JsValue> {
    panic::set_hook(Box::new(console_error_panic_hook::hook));

    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;

    let context = canvas
        .get_context("webgl")?
        .unwrap()
        .dyn_into::<WebGlRenderingContext>()?;
    
    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    let mut context = Context::new(context);

    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {

        game.update(&mut context).expect("Error while updating");
        game.draw(&mut context).expect("Error while drawing");

        request_animation_frame(f.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut()>));

    request_animation_frame(g.borrow().as_ref().unwrap());

    Ok(())
}

fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}
