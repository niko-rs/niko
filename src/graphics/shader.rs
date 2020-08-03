use crate::{Error, NikoError, graphics::{ShaderError, check_error, ShaderId, ProgramId}};
use glow::HasContext;
use std::collections::HashMap;

macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

unsafe fn compile_shader(gl: &glow::Context, source: &str, kind: u32) -> Result<ShaderId, Error> {
    let shader = gl.create_shader(kind)
        .map_err(|error| NikoError::PlatformError(error))?;

    gl.shader_source(shader, source);
    gl.compile_shader(shader);

    if !gl.get_shader_compile_status(shader) {
        let error = gl.get_shader_info_log(shader);
        gl.delete_shader(shader);
        return Err(ShaderError::ShaderCompileError(error).into());
    }

    Ok(shader)
}

unsafe fn reflect_attributes(gl: &glow::Context, program: ProgramId) -> HashMap<String, u32> {
    let mut attributes = HashMap::new();
    let attribute_count = gl.get_active_attributes(program);
    for index in 0..attribute_count {
        if let Some(attribute) = gl.get_active_attribute(program, index) {
            log!("attribute found at {}: {}, {}, {}", index, attribute.name, attribute.atype, attribute.size);
            attributes.insert(attribute.name, index);
        } else {
            log!("no attribute found at {}", index);
        }
    }

    attributes
}

unsafe fn reflect_uniforms(gl: &glow::Context, program: ProgramId) -> HashMap<String, glow::UniformLocation> {
    let mut uniforms = HashMap::new();
    let uniform_count = gl.get_active_uniforms(program);
    for index in 0..uniform_count {
        if let Some(uniform) = gl.get_active_uniform(program, index) {
            if let Some(location) = gl.get_uniform_location(program, &uniform.name) {
                log!("uniform found at {}: {}, {}, {}", index, uniform.name, uniform.utype, uniform.size);
                uniforms.insert(uniform.name, location);
            } else {
                log!("no uniform found at {}", index);
            }
        }
    }

    uniforms
}

unsafe fn build_program(gl: &glow::Context, vertex_shader_source: &str, fragment_shader_source: &str) -> Result<ProgramId, Error> {
    let vertex_shader = compile_shader(gl, vertex_shader_source, glow::VERTEX_SHADER)?;
    let fragment_shader = compile_shader(gl, fragment_shader_source, glow::FRAGMENT_SHADER)?;
    
    let program = gl.create_program()
        .map_err(|error| NikoError::PlatformError(error))?;

    gl.attach_shader(program, vertex_shader);
    gl.attach_shader(program, fragment_shader);
    
    gl.link_program(program);

    if !gl.get_program_link_status(program) {
        let error = gl.get_program_info_log(program);
        gl.delete_program(program);
        return Err(NikoError::PlatformError(error).into());
    }

    gl.detach_shader(program, vertex_shader);
    gl.detach_shader(program, fragment_shader);

    gl.delete_shader(vertex_shader);
    gl.delete_shader(fragment_shader);

    Ok(program)
}

#[derive(Debug)]
pub struct Shader {
    inner: ProgramId,
    attributes: HashMap<String, u32>,
    uniforms: HashMap<String, glow::UniformLocation>,
}

impl Shader {
    pub fn create(gl: &glow::Context, vertex_shader: &str, fragment_shader: &str) -> Result<Self, Error> {
        let inner = unsafe { build_program(gl, vertex_shader, fragment_shader)? };

        let attributes = unsafe { reflect_attributes(gl, inner) };
        let uniforms = unsafe { reflect_uniforms(gl, inner) };

        check_error(gl)?;

        Ok(Self {
            inner,
            attributes,
            uniforms,
        })
    }

    pub fn get_attribute_location(&self, name: &str) -> Option<u32> {
        match self.attributes.get(name) {
            Some(id) => Some(*id),
            None => None,
        }
    }

    pub fn query_attribute_location(&self, gl: &glow::Context, name: &str) -> Option<u32> {
        let result = unsafe {
            gl.get_attrib_location(self.inner, name)
        };

        result
    }

    pub fn get_uniform_location(&self, name: &str) -> Option<&glow::UniformLocation> {
        self.uniforms.get(name)
    }

    pub fn query_uniform_location(&self, gl: &glow::Context, name: &str) -> Option<glow::UniformLocation> {
        let result = unsafe {
            gl.get_uniform_location(self.inner, name)
        };

        result
    }

    pub(crate) fn get_inner(&self) -> ProgramId {
        self.inner
    }
}
