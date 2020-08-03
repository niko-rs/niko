use std::rc::Rc;
use std::cell::RefCell;
use std::collections::{HashMap, VecDeque};
use web_sys::{HtmlImageElement};
use crate::{Error, NikoError, Event, event, graphics::TextureId};
use glow::HasContext;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
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
    textures: HashMap<u32, TextureId>,
    sizes: HashMap<u32, (u32, u32)>,
    next_id: u32,
}

impl Images {
    pub fn new() -> Self {
        Self {
            images: HashMap::new(),
            textures: HashMap::new(),
            sizes: HashMap::new(),
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

    #[cfg(target_arch = "wasm32")]
    pub(crate) fn finish_loading(&mut self, id: u32, gl: &glow::Context) -> Result<(), Error> {
        let image = self.images.get(&id).unwrap();
        let width = image.width();
        let height = image.height();
        self.sizes.insert(id, (width, height));

        let texture = unsafe {
            let texture = gl.create_texture()
                .map_err(|error| NikoError::PlatformError(error))?;

            gl.bind_texture(glow::TEXTURE_2D, Some(texture));
            gl.tex_image_2d_with_html_image(
                glow::TEXTURE_2D,
                0,
                glow::RGBA as i32,
                glow::RGBA,
                glow::UNSIGNED_BYTE,
                &image,
            );

            gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_WRAP_S, glow::CLAMP_TO_EDGE as i32);
            gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_WRAP_T, glow::CLAMP_TO_EDGE as i32);
            gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_MIN_FILTER, glow::LINEAR as i32);

            texture
        };

        self.textures.insert(id, texture);

        Ok(())
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) fn finish_loading(&mut self, id: u32, gl: &glow::Context) -> Result<(), Error> {
        unimplemented!()
    }

    pub fn find_texture(&self, image: Image) -> Option<TextureId> {
        match self.textures.get(&image.id) {
            Some(texture) => Some(*texture),
            None => None,
        }
    }

    pub fn find_size(&self, image: Image) -> Option<(u32, u32)> {
        match self.sizes.get(&image.id) {
            Some((width, height)) => Some((*width, *height)),
            None => None,
        }
    }
}
