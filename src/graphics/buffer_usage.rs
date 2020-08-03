use glow;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum BufferUsage {
    StaticDraw,
    DynamicDraw,
}

impl Into<u32> for BufferUsage {
    fn into(self) -> u32 {
        match self {
            BufferUsage::StaticDraw => glow::STATIC_DRAW,
            BufferUsage::DynamicDraw => glow::DYNAMIC_DRAW,
        }
    }
}
