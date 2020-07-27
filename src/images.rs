use std::rc::Rc;
use std::cell::RefCell;
use std::collections::{HashMap, VecDeque};
use web_sys::{HtmlImageElement, WebGl2RenderingContext, WebGlTexture};
use crate::{Error, NikoError, Event, event};

pub struct Image {
    pub(crate) id: u32,
}

impl Image {
    pub(crate) fn new(id: u32) -> Self {
        Self {
            id,
        }
    }
}

pub struct Images {
    images: HashMap<u32, HtmlImageElement>,
    textures: HashMap<u32, WebGlTexture>,
    next_id: u32,
}

impl Images {
    pub fn new() -> Self {
        Self {
            images: HashMap::new(),
            textures: HashMap::new(),
            next_id: 0,
        }
    }

    pub(crate) fn create_image_from_url(&mut self, url: &str, event_queue_handle: Rc<RefCell<VecDeque<Event>>>) -> Result<Image, Error> {
        let id = self.next_id;
        self.next_id += 1;

        let html_image = HtmlImageElement::new().map_err(|_| NikoError::PlatformError("could not create a new HtmlImageElement".to_string()))?;
        html_image.set_src(url);

        let closure = event(&html_image, "load", move |_event: web_sys::Event| {
            event_queue_handle.borrow_mut().push_back(Event::ImageLoaded(id));
        }).map_err(|_| NikoError::PlatformError("could not create a loading closure for an image".to_string()))?;
        closure.forget();

        self.images.insert(id, html_image);

        Ok(Image::new(id))
    }

    pub(crate) fn finish_loading(&mut self, id: u32, gl: &mut WebGl2RenderingContext) -> Result<(), Error> {

        let image = self.images.get(&id).unwrap();

        let texture = gl.create_texture().unwrap();
        gl.bind_texture(WebGl2RenderingContext::TEXTURE_2D, Some(&texture));

        let width = image.width();
        let height = image.height();

        gl.tex_image_2d_with_u32_and_u32_and_html_image_element(
            WebGl2RenderingContext::TEXTURE_2D, // target
            0, // level
            WebGl2RenderingContext::RGBA as i32, // internalformat ???
            WebGl2RenderingContext::RGBA, // format
            WebGl2RenderingContext::UNSIGNED_BYTE, // type
            &image, // object
        ).unwrap();

        gl.tex_parameteri(WebGl2RenderingContext::TEXTURE_2D, WebGl2RenderingContext::TEXTURE_WRAP_S, WebGl2RenderingContext::CLAMP_TO_EDGE as i32);
        gl.tex_parameteri(WebGl2RenderingContext::TEXTURE_2D, WebGl2RenderingContext::TEXTURE_WRAP_T, WebGl2RenderingContext::CLAMP_TO_EDGE as i32);
        gl.tex_parameteri(WebGl2RenderingContext::TEXTURE_2D, WebGl2RenderingContext::TEXTURE_MIN_FILTER, WebGl2RenderingContext::LINEAR as i32);

        self.textures.insert(id, texture);

        Ok(())
    }

    pub(crate) fn bind_texture(&mut self, id: u32, gl: &mut WebGl2RenderingContext) -> Result<(), Error> {
        if let Some(texture) = self.textures.get(&id) {
            gl.bind_texture(WebGl2RenderingContext::TEXTURE_2D, Some(texture));
        }

        Ok(())
    }
}
