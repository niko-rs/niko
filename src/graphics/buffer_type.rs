use glow;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum BufferType {
    VertexBuffer,
    IndexBuffer,
}

impl Into<u32> for BufferType {
    fn into(self) -> u32 {
        match self {
            BufferType::VertexBuffer => glow::ARRAY_BUFFER,
            BufferType::IndexBuffer => glow::ELEMENT_ARRAY_BUFFER,
        }
    }
}
