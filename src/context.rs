use web_sys::WebGlRenderingContext;

pub struct Context {
    pub(crate) rendering_context: WebGlRenderingContext,
}

impl Context {
    pub(crate) fn new(rendering_context: WebGlRenderingContext) -> Self {
        Self {
            rendering_context,
        }
    }
}
