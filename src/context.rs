use std::cell::RefCell;
use std::rc::Rc;
use std::collections::VecDeque;
use web_sys::{WebGl2RenderingContext};
use crate::{Event, Images, shader::Shader, buffer::Buffer};

pub struct Context {
    pub(crate) rendering_context: WebGl2RenderingContext,
    pub(crate) event_queue: Rc<RefCell<VecDeque<Event>>>,
    pub(crate) images: Images,
    pub(crate) shader: Shader,
    pub(crate) buffer: Buffer,
}

impl Context {
    pub(crate) fn new(
        rendering_context: WebGl2RenderingContext,
        event_queue: Rc<RefCell<VecDeque<Event>>>,
        images: Images,
        shader: Shader,
        buffer: Buffer,
    ) -> Self {
        Self {
            rendering_context,
            event_queue,
            images,
            shader,
            buffer,
        }
    }
}
