mod context;
mod error;
mod game;
mod images;
mod key;
mod log;
mod shader;
mod buffer;

mod rectangle;
pub use rectangle::*;
mod point;
pub use point::*;

pub use context::*;
pub use error::*;
pub use log::*;
pub use game::*;
pub use images::*;
pub use key::*;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen::convert::FromWasmAbi;
use std::cell::RefCell;
use std::rc::Rc;
use std::panic;
use std::collections::VecDeque;
use web_sys::{
    WebGl2RenderingContext,
    KeyboardEvent,
    EventTarget,
};

#[derive(Debug)]
pub enum Event {
    KeyDown(Key),
    KeyUp(Key),
    ImageLoaded(u32),
}

pub fn clear(context: &mut Context, r: f32, g: f32, b: f32, a: f32) {
    context.rendering_context.clear_color(r, g, b, a);
    context.rendering_context.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);
}

pub fn run(mut game: Box<dyn Game>) -> Result<(), JsValue> {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;
    let event_queue = Rc::new(RefCell::new(VecDeque::new()));
    
    // Key down events
    let event_queue_handle = Rc::clone(&event_queue);
    let closure = event(&canvas, "keydown", move |event: KeyboardEvent| {
        event.prevent_default();
        event.stop_propagation();
        if let Some(key) = into_key(event) {
            event_queue_handle.borrow_mut().push_back(Event::KeyDown(key));
        }
    }).unwrap();
    closure.forget();

    // Key up events
    let event_queue_handle = Rc::clone(&event_queue);
    let closure = event(&canvas, "keyup", move |event: KeyboardEvent| {
        event.prevent_default();
        event.stop_propagation();
        if let Some(key) = into_key(event) {
            event_queue_handle.borrow_mut().push_back(Event::KeyUp(key));
        }
    }).unwrap();
    closure.forget();

    let mut gl = canvas
        .get_context("webgl2")?
        .unwrap()
        .dyn_into::<WebGl2RenderingContext>()?;
    
    let shader = shader::create_shader(&mut gl).expect("could not create shader");
    let buffer = buffer::create_buffer(&mut gl).expect("could not create buffer");

    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    let mut context = Context::new(gl, event_queue, Images::new(), shader, buffer);

    game.initialize(&mut context).expect("Error while initializing");

    // game loop
    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        while let Some(event) = context.event_queue.borrow_mut().pop_front() {
            match event {
                Event::ImageLoaded(id) => {
                    context.images.finish_loading(id, &mut context.rendering_context).expect("could not finish loading an image");
                }
                _ => {},
            }
        }

        game.update(&mut context).expect("Error while updating");
        game.draw(&mut context).expect("Error while drawing");

        request_animation_frame(f.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut()>));

    request_animation_frame(g.borrow().as_ref().unwrap());

    Ok(())
}

pub fn draw_image(context: &mut Context, image: &Image) -> Result<(), Error> {
    let gl = &mut context.rendering_context;
    gl.viewport(0, 0, 1280, 720);
    gl.use_program(Some(&context.shader.program));

    gl.enable(WebGl2RenderingContext::BLEND);
    gl.blend_func(WebGl2RenderingContext::SRC_ALPHA, WebGl2RenderingContext::ONE_MINUS_SRC_ALPHA);

    gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&context.buffer.position_buffer));
    gl.vertex_attrib_pointer_with_i32(context.shader.position_attribute, 4, WebGl2RenderingContext::FLOAT, false, 0, 0);
    gl.enable_vertex_attrib_array(context.shader.position_attribute);

    gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&context.buffer.uv_buffer));
    gl.vertex_attrib_pointer_with_i32(context.shader.uv_attribute, 2, WebGl2RenderingContext::FLOAT, false, 0, 0);
    gl.enable_vertex_attrib_array(context.shader.uv_attribute);

    gl.bind_buffer(WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER, Some(&context.buffer.index_buffer));

    gl.active_texture(WebGl2RenderingContext::TEXTURE0);

    context.images.bind_texture(image.id, gl).unwrap();

    gl.uniform1i(Some(&context.shader.sampler_uniform), 0);

    gl.draw_elements_with_i32(WebGl2RenderingContext::TRIANGLES, 6, WebGl2RenderingContext::UNSIGNED_SHORT, 0);

    Ok(())
}

pub fn load_image(context: &mut Context, url: &str) -> Result<Image, Error> {
    let event_queue_handle = Rc::clone(&context.event_queue);
    context.images.create_image_from_url(url, event_queue_handle)
}

fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

fn event<F, T>(target: &EventTarget, event_name: &str, callback: F) -> Result<Closure<dyn FnMut(T)>, Error>
where
    F: FnMut(T) + 'static,
    T: FromWasmAbi + 'static,
{
    let callback = Closure::wrap(Box::new(callback) as Box<dyn FnMut(T)>);

    target.add_event_listener_with_callback(event_name, callback.as_ref().unchecked_ref()).unwrap();

    Ok(callback)
}

fn into_key(event: KeyboardEvent) -> Option<Key> {
    match (event.key().as_ref(), event.location()) {
        ("a", _) | ("A", _) => Some(Key::A),
        ("b", _) | ("B", _) => Some(Key::B),
        ("c", _) | ("C", _) => Some(Key::C),
        ("d", _) | ("D", _) => Some(Key::D),
        ("e", _) | ("E", _) => Some(Key::E),
        ("f", _) | ("F", _) => Some(Key::F),
        ("g", _) | ("G", _) => Some(Key::G),
        ("h", _) | ("H", _) => Some(Key::H),
        ("i", _) | ("I", _) => Some(Key::I),
        ("j", _) | ("J", _) => Some(Key::J),
        ("k", _) | ("K", _) => Some(Key::K),
        ("l", _) | ("L", _) => Some(Key::L),
        ("m", _) | ("M", _) => Some(Key::M),
        ("n", _) | ("N", _) => Some(Key::N),
        ("o", _) | ("O", _) => Some(Key::O),
        ("p", _) | ("P", _) => Some(Key::P),
        ("q", _) | ("Q", _) => Some(Key::Q),
        ("r", _) | ("R", _) => Some(Key::R),
        ("s", _) | ("S", _) => Some(Key::S),
        ("t", _) | ("T", _) => Some(Key::T),
        ("u", _) | ("U", _) => Some(Key::U),
        ("v", _) | ("V", _) => Some(Key::V),
        ("w", _) | ("W", _) => Some(Key::W),
        ("x", _) | ("X", _) => Some(Key::X),
        ("y", _) | ("Y", _) => Some(Key::Y),
        ("z", _) | ("Z", _) => Some(Key::Z),

        ("0", KeyboardEvent::DOM_KEY_LOCATION_STANDARD) => Some(Key::D0),
        ("1", KeyboardEvent::DOM_KEY_LOCATION_STANDARD) => Some(Key::D1),
        ("2", KeyboardEvent::DOM_KEY_LOCATION_STANDARD) => Some(Key::D2),
        ("3", KeyboardEvent::DOM_KEY_LOCATION_STANDARD) => Some(Key::D3),
        ("4", KeyboardEvent::DOM_KEY_LOCATION_STANDARD) => Some(Key::D4),
        ("5", KeyboardEvent::DOM_KEY_LOCATION_STANDARD) => Some(Key::D5),
        ("6", KeyboardEvent::DOM_KEY_LOCATION_STANDARD) => Some(Key::D6),
        ("7", KeyboardEvent::DOM_KEY_LOCATION_STANDARD) => Some(Key::D7),
        ("8", KeyboardEvent::DOM_KEY_LOCATION_STANDARD) => Some(Key::D8),
        ("9", KeyboardEvent::DOM_KEY_LOCATION_STANDARD) => Some(Key::D9),

        ("F1", _) => Some(Key::F1),
        ("F2", _) => Some(Key::F2),
        ("F3", _) => Some(Key::F3),
        ("F4", _) => Some(Key::F4),
        ("F5", _) => Some(Key::F5),
        ("F6", _) => Some(Key::F6),
        ("F7", _) => Some(Key::F7),
        ("F8", _) => Some(Key::F8),
        ("F9", _) => Some(Key::F9),
        ("F10", _) => Some(Key::F10),
        ("F11", _) => Some(Key::F11),
        ("F12", _) => Some(Key::F12),
        ("F13", _) => Some(Key::F13),
        ("F14", _) => Some(Key::F14),
        ("F15", _) => Some(Key::F15),
        ("F16", _) => Some(Key::F16),
        ("F17", _) => Some(Key::F17),
        ("F18", _) => Some(Key::F18),
        ("F19", _) => Some(Key::F19),
        ("F20", _) => Some(Key::F20),
        ("F21", _) => Some(Key::F21),
        ("F22", _) => Some(Key::F22),
        ("F23", _) => Some(Key::F23),
        ("F24", _) => Some(Key::F24),

        ("NumLock", _) => Some(Key::NumLock),
        ("0", KeyboardEvent::DOM_KEY_LOCATION_NUMPAD) => Some(Key::NumPad1),
        ("1", KeyboardEvent::DOM_KEY_LOCATION_NUMPAD) => Some(Key::NumPad2),
        ("2", KeyboardEvent::DOM_KEY_LOCATION_NUMPAD) => Some(Key::NumPad3),
        ("3", KeyboardEvent::DOM_KEY_LOCATION_NUMPAD) => Some(Key::NumPad4),
        ("4", KeyboardEvent::DOM_KEY_LOCATION_NUMPAD) => Some(Key::NumPad5),
        ("5", KeyboardEvent::DOM_KEY_LOCATION_NUMPAD) => Some(Key::NumPad6),
        ("6", KeyboardEvent::DOM_KEY_LOCATION_NUMPAD) => Some(Key::NumPad7),
        ("7", KeyboardEvent::DOM_KEY_LOCATION_NUMPAD) => Some(Key::NumPad8),
        ("8", KeyboardEvent::DOM_KEY_LOCATION_NUMPAD) => Some(Key::NumPad9),
        ("9", KeyboardEvent::DOM_KEY_LOCATION_NUMPAD) => Some(Key::NumPad0),
        ("+", KeyboardEvent::DOM_KEY_LOCATION_NUMPAD) => Some(Key::OemPlus), // TODO NumPad
        ("-", KeyboardEvent::DOM_KEY_LOCATION_NUMPAD) => Some(Key::OemMinus), // TODO NumPad
        ("*", KeyboardEvent::DOM_KEY_LOCATION_NUMPAD) => Some(Key::Multiply), // TODO NumPad
        ("/", KeyboardEvent::DOM_KEY_LOCATION_NUMPAD) => Some(Key::Divide), // TODO NumPad
        ("Enter", KeyboardEvent::DOM_KEY_LOCATION_NUMPAD) => Some(Key::Enter), // TODO NumPad

        ("Control", KeyboardEvent::DOM_KEY_LOCATION_LEFT) => Some(Key::LeftControl),
        ("Shift", KeyboardEvent::DOM_KEY_LOCATION_LEFT) => Some(Key::LeftShift),
        ("Alt", KeyboardEvent::DOM_KEY_LOCATION_LEFT) => Some(Key::LeftAlt),
        ("Control", KeyboardEvent::DOM_KEY_LOCATION_RIGHT) => Some(Key::RightControl),
        ("Shift", KeyboardEvent::DOM_KEY_LOCATION_RIGHT) => Some(Key::RightShift),
        ("Alt", KeyboardEvent::DOM_KEY_LOCATION_RIGHT) => Some(Key::RightAlt),

        ("ArrowUp", _) => Some(Key::Up),
        ("ArrowDown", _) => Some(Key::Down),
        ("ArrowLeft", _) => Some(Key::Left),
        ("ArrowRight", _) => Some(Key::Right),

        ("&", _) => Some(Key::None), // TODO
        ("*", KeyboardEvent::DOM_KEY_LOCATION_STANDARD) => Some(Key::None), // TODO
        ("@", _) => Some(Key::None), // TODO
        ("`", _) => Some(Key::None), // TODO
        ("\\", _) => Some(Key::None), // TODO
        ("Backspace", _) => Some(Key::None), // TODO
        ("CapsLock", _) => Some(Key::CapsLock),
        ("^", _) => Some(Key::None), // TODO
        (":", _) => Some(Key::None), // TODO
        (",", _) => Some(Key::None), // TODO
        ("Delete", _) => Some(Key::Delete),
        ("$", _) => Some(Key::None), // TODO
        ("\"", _) => Some(Key::None), // TODO
        ("End", _) => Some(Key::End),
        ("Enter", KeyboardEvent::DOM_KEY_LOCATION_STANDARD) => Some(Key::Enter),
        ("=", _) => Some(Key::None), // TODO
        ("Escape", _) => Some(Key::Escape),
        ("!", _) => Some(Key::None), // TODO
        (">", _) => Some(Key::None), // TODO
        ("#", _) => Some(Key::None), // TODO
        ("Home", _) => Some(Key::Home),
        ("Insert", _) => Some(Key::Insert),
        ("{", _) => Some(Key::None), // TODO
        ("(", _) => Some(Key::None), // TODO
        ("<", _) => Some(Key::None), // TODO
        ("-", KeyboardEvent::DOM_KEY_LOCATION_STANDARD) => Some(Key::None), // TODO
        ("PageDown", _) => Some(Key::PageDown),
        ("PageUp", _) => Some(Key::PageUp),
        ("Pause", _) => Some(Key::Pause),
        ("%", _) => Some(Key::None), // TODO
        (".", _) => Some(Key::None), // TODO
        ("+", KeyboardEvent::DOM_KEY_LOCATION_STANDARD) => Some(Key::None), // TODO
        ("PrintScreen", _) => Some(Key::PrintScreen),
        ("?", _) => Some(Key::None), // TODO
        ("'", _) => Some(Key::None), // TODO
        ("}", _) => Some(Key::None), // TODO
        (")", _) => Some(Key::None), // TODO
        ("ScrollLock", _) => Some(Key::None), // TODO
        (";", _) => Some(Key::None), // TODO
        ("/", KeyboardEvent::DOM_KEY_LOCATION_STANDARD) => Some(Key::None), // TODO
        (" ", _) => Some(Key::Space),
        ("Tab", _) => Some(Key::Tab),
        ("_", _) => Some(Key::None), // TODO

        _ => None,
    }
}
