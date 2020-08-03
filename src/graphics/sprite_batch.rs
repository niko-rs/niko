use crate::{
    Error,
    Rectangle,
    Context,
    Image,
    Color,
    graphics::{
        Buffer,
        BufferType,
        BufferUsage,
    },
};
use glow::HasContext;

struct DynamicBuffer {
    vertex_data: Vec<f32>,
    vertices: u16,
    indices: Vec<u16>,
}

impl DynamicBuffer {
    pub fn new() -> Self {
        Self {
            vertex_data: Vec::new(),
            vertices: 0,
            indices: Vec::new(),
        }
    }

    pub fn push_vertex(&mut self, x: f32, y: f32, u: f32, v: f32) -> u16 {
        let index = self.vertices;
        self.vertices += 1;

        self.vertex_data.push(x);
        self.vertex_data.push(y);
        self.vertex_data.push(u);
        self.vertex_data.push(v);

        index
    }

    pub fn push_quad(&mut self, a: u16, b: u16, c: u16, d: u16) {
        self.indices.push(a);
        self.indices.push(b);
        self.indices.push(c);
        
        self.indices.push(a);
        self.indices.push(c);
        self.indices.push(d);
    }

    pub fn build(self, gl: &glow::Context) -> Result<(i32, Buffer, Buffer), Error> {
        use std::mem::size_of;
        use std::slice::from_raw_parts;

        let vertex_buffer = unsafe {
            let byte_len = self.vertex_data.len() * size_of::<f32>();
            let byte_data = from_raw_parts(self.vertex_data.as_ptr() as *const u8, byte_len);
            Buffer::create(gl, BufferType::VertexBuffer, BufferUsage::DynamicDraw, &byte_data)?
        };

        let index_buffer = unsafe {
            let byte_len = self.indices.len() * size_of::<u16>();
            let byte_data = from_raw_parts(self.indices.as_ptr() as *const u8, byte_len);
            Buffer::create(gl, BufferType::IndexBuffer, BufferUsage::DynamicDraw, &byte_data)?
        };

        let count = self.indices.len() as i32;

        Ok((count, vertex_buffer, index_buffer))
    }
}

struct SpriteInstance {
    sprite: Image,
    source: Rectangle,
    target: Rectangle,
    color: Color,
}

pub struct SpriteBatch {
    instances: Vec<SpriteInstance>,
    brute_force: bool,
}

impl SpriteBatch {
    pub fn new() -> Self {
        Self {
            instances: Vec::new(),
            brute_force: false,
        }
    }

    pub fn draw_sprite(&mut self, sprite: Image, source: Rectangle, target: Rectangle, color: Color) {
        // TODO expose color
        self.instances.push(SpriteInstance {
            sprite,
            source,
            target,
            color,
        });
    }

    pub fn draw(self, context: &mut Context) -> Result<(), Error> {
        if self.instances.len() < 1 {
            return Ok(());
        }

        let gl = &context.gl;
        let shader = &context.sprite_shader;

        let canvas_size = Rectangle::new(0, 0, 1280, 720);

        let mut dynamic_buffer = DynamicBuffer::new();
        for instance in &self.instances {

            // TODO propper ignore unloaded sprites
            let image_size = if let Some((width, height)) = context.images.find_size(instance.sprite) {
                Rectangle::new(0, 0, width as i32, height as i32)
            } else {
                Rectangle::new(0, 0, 1, 1)
            };

            let (source_left, source_right, source_top, source_bottom) = instance.source.to_rendering_position(&image_size);
            let (target_left, target_right, target_top, target_bottom) = instance.target.to_rendering_position(&canvas_size);

            let a = dynamic_buffer.push_vertex((target_left - 0.5) * 2.0, (target_bottom - 0.5) * 2.0, source_left, source_top);
            let b = dynamic_buffer.push_vertex((target_right - 0.5) * 2.0, (target_bottom - 0.5) * 2.0, source_right, source_top);
            let c = dynamic_buffer.push_vertex((target_right - 0.5) * 2.0,  (target_top - 0.5) * 2.0, source_right, source_bottom);
            let d = dynamic_buffer.push_vertex((target_left - 0.5) * 2.0,  (target_top - 0.5) * 2.0, source_left, source_bottom);

            dynamic_buffer.push_quad(a, b, c, d);

        }
        let (_count, vertex_buffer, index_buffer) = dynamic_buffer.build(gl)?;
        
        unsafe {
            gl.enable(glow::BLEND);
            gl.blend_func(glow::SRC_ALPHA, glow::ONE_MINUS_SRC_ALPHA);

            gl.use_program(Some(shader.get_inner()));
            
            let position_attribute = shader.get_attribute_location("position")
                .ok_or(super::ShaderError::AttributeNotFound("position".to_string()))?;
            gl.bind_buffer(glow::ARRAY_BUFFER, Some(vertex_buffer.get_inner()));
            gl.vertex_attrib_pointer_f32(position_attribute, 4, glow::FLOAT, false, 0, 0);
            gl.enable_vertex_attrib_array(position_attribute);
            
            gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(index_buffer.get_inner()));
            
            let color_location = shader.get_uniform_location("color")
                .ok_or(super::ShaderError::UniformNotFound("color".to_string()))?;
            let sprite_location = shader.get_uniform_location("sprite")
                .ok_or(super::ShaderError::UniformNotFound("sprite".to_string()))?;

            let mut draw_calls = 0;
            let mut skipped = 0;

            if self.brute_force {
                // draw brute-force
                let mut offset = 0;
                for instance in &self.instances {
                    if let Some(sprite) = context.images.find_texture(instance.sprite) {
                        gl.active_texture(glow::TEXTURE0);
                        gl.bind_texture(glow::TEXTURE_2D, Some(sprite));
                        gl.uniform_1_i32(Some(sprite_location), 0);
                        let (r, g, b, a) = instance.color.into_normalized();
                        gl.uniform_4_f32(Some(color_location), r, g, b, a);
                        gl.draw_elements(glow::TRIANGLES, 6, glow::UNSIGNED_SHORT, offset);
                        offset += 6 * std::mem::size_of::<u16>() as i32;

                        draw_calls += 1;
                    } else {
                        skipped += 1;
                    }
                }
            } else {
                // draw batched

                // TODO ignore sprites outside of view rectangle

                // begin first batch
                let mut batch_offset = 0;
                let mut current_offset = 0;
                let mut draw_count = 0;
                let mut last_color = self.instances[0].color;
                let mut last_sprite = self.instances[0].sprite;

                for instance in &self.instances {
                    // check if we have to finish current batch and start next batch
                    if last_color != instance.color || last_sprite != instance.sprite {
                        // draw current batch
                        if let Some(sprite) = context.images.find_texture(last_sprite) {
                            gl.active_texture(glow::TEXTURE0);
                            gl.bind_texture(glow::TEXTURE_2D, Some(sprite));
                            gl.uniform_1_i32(Some(sprite_location), 0);
                            let (r, g, b, a) = last_color.into_normalized();
                            gl.uniform_4_f32(Some(color_location), r, g, b, a);
                            gl.draw_elements(glow::TRIANGLES, draw_count, glow::UNSIGNED_SHORT, batch_offset);

                            draw_calls += 1;
                        } else {
                            skipped += 1;
                        }
    
                        //begin new batch
                        draw_count = 0;
                        batch_offset = current_offset;
                        last_color = instance.color;
                        last_sprite = instance.sprite;
                    }

                    // continue batching
                    current_offset += 6 * std::mem::size_of::<u16>() as i32;
                    draw_count += 6;
                }
    
                // draw last batch
                if let Some(sprite) = context.images.find_texture(last_sprite) {
                    gl.active_texture(glow::TEXTURE0);
                    gl.bind_texture(glow::TEXTURE_2D, Some(sprite));
                    gl.uniform_1_i32(Some(sprite_location), 0);
                    let (r, g, b, a) = last_color.into_normalized();
                    gl.uniform_4_f32(Some(color_location), r, g, b, a);
                    gl.draw_elements(glow::TRIANGLES, draw_count, glow::UNSIGNED_SHORT, batch_offset);
                    draw_calls += 1;
                } else {
                    skipped += 1;
                }
            }

            gl.delete_buffer(vertex_buffer.get_inner());
            gl.delete_buffer(index_buffer.get_inner());

            context.metrics.add_draw_calls(draw_calls);
            context.metrics.add_sprites_drawn(self.instances.len() - skipped);
            context.metrics.add_sprites_skipped(skipped);
        }

        Ok(())
    }
}
