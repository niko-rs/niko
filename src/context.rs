use std::cell::RefCell;
use std::rc::Rc;
use std::collections::VecDeque;
use glow::Context as GlowContext;
use crate::{
    Event,
    Images,
    graphics::Shader,
};

pub struct Context {
    pub(crate) gl: GlowContext,
    pub(crate) event_queue: Rc<RefCell<VecDeque<Event>>>,
    pub(crate) images: Images,
    pub(crate) sprite_shader: Shader,
}

impl Context {
    pub(crate) fn new(
        gl: GlowContext,
        event_queue: Rc<RefCell<VecDeque<Event>>>,
        images: Images,
        sprite_shader: Shader,
    ) -> Self {
        Self {
            gl,
            event_queue,
            images,
            sprite_shader,
        }
    }
}
