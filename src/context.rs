use std::cell::RefCell;
use std::rc::Rc;
use std::collections::VecDeque;
use glow::Context as GlowContext;
use crate::{
    Event,
    Images,
    Input,
    Metrics,
    graphics::Shader,
};

pub struct Context {
    pub(crate) gl: GlowContext,
    pub(crate) event_queue: Rc<RefCell<VecDeque<Event>>>,
    pub(crate) images: Images,
    pub(crate) input: Input,
    pub(crate) metrics: Metrics,
    pub(crate) sprite_shader: Shader,
}

impl Context {
    pub(crate) fn new(
        gl: GlowContext,
        event_queue: Rc<RefCell<VecDeque<Event>>>,
        images: Images,
        input: Input,
        metrics: Metrics,
        sprite_shader: Shader,
    ) -> Self {
        Self {
            gl,
            event_queue,
            images,
            input,
            metrics,
            sprite_shader,
        }
    }
}
