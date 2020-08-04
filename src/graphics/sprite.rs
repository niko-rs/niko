use crate::{Error, Image, Rectangle, Point, Context};
use std::rc::Rc;

pub fn load_sprite(context: &mut Context, url: &str, size: Point) -> Result<Sprite, Error> {
    let event_queue_handle = Rc::clone(&context.event_queue);
    let image = context.images.create_image_from_url(url, event_queue_handle)?;
    Ok(Sprite::new(image, Rectangle::new(0, 0, size.x, size.y)))
}

pub fn load_sprite_sheet(context: &mut Context, url: &str, sprite_size: Point, sheet_size: Point) -> Result<Vec<Sprite>, Error> {
    let event_queue_handle = Rc::clone(&context.event_queue);
    let image = context.images.create_image_from_url(url, event_queue_handle)?;

    let num_sprites = sheet_size / sprite_size;

    let mut sprites = Vec::new();
    for y in 0..num_sprites.y {
        for x in 0..num_sprites.x {
            sprites.push(Sprite::new(
                image,
                Rectangle::new(
                    sprite_size.x * x,
                    sprite_size.y * y,
                    sprite_size.x,
                    sprite_size.y,
                ),
            ));
        }
    }

    Ok(sprites)
}

#[derive(Debug, Clone)]
pub struct Sprite {
    pub(crate) image: Image,
    pub(crate) area: Rectangle,
}

impl Sprite {
    pub fn new(image: Image, area: Rectangle) -> Self {
        Self {
            image,
            area,
        }
    }
}
