use web_sys::{WebGl2RenderingContext, WebGlBuffer};

pub struct Buffer {
    pub(crate) position_buffer: WebGlBuffer,
    pub(crate) uv_buffer: WebGlBuffer,
    pub(crate) index_buffer: WebGlBuffer,
}

pub fn create_buffer(gl: &mut WebGl2RenderingContext) -> Result<Buffer, ()> {
    let positions: [f32; 16] = [
        -1.0, -1.0, 0.0, 1.0,
         1.0, -1.0, 0.0, 1.0,
         1.0,  1.0, 0.0, 1.0,
        -1.0,  1.0, 0.0, 1.0,
    ];

    let position_buffer = gl.create_buffer().expect("could not create buffer");
    gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&position_buffer));
    unsafe {
        let vert_array = js_sys::Float32Array::view(&positions);
        gl.buffer_data_with_array_buffer_view(
            WebGl2RenderingContext::ARRAY_BUFFER,
            &vert_array,
            WebGl2RenderingContext::STATIC_DRAW,
        );
    }

    let uvs: [f32; 8] = [
        0.0, 1.0,
        1.0, 1.0,
        1.0, 0.0,
        0.0, 0.0,
    ];

    let uv_buffer = gl.create_buffer().expect("could not create buffer");
    gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&uv_buffer));
    unsafe {
        let vert_array = js_sys::Float32Array::view(&uvs);
        gl.buffer_data_with_array_buffer_view(
            WebGl2RenderingContext::ARRAY_BUFFER,
            &vert_array,
            WebGl2RenderingContext::STATIC_DRAW,
        );
    }

    let indices: [u16; 6] = [
        0, 1, 2,
        0, 2, 3,
    ];

    let index_buffer = gl.create_buffer().expect("could not create buffer");
    gl.bind_buffer(WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER, Some(&index_buffer));
    unsafe {
        let vert_array = js_sys::Uint16Array::view(&indices);
        gl.buffer_data_with_array_buffer_view(
            WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER,
            &vert_array,
            WebGl2RenderingContext::STATIC_DRAW,
        );
    }

    Ok(Buffer {
        position_buffer,
        uv_buffer,
        index_buffer,
    })
}
