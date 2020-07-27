use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{WebGlProgram, WebGl2RenderingContext, WebGlShader, WebGlUniformLocation};

pub struct Shader {
    pub(crate) program: WebGlProgram,
    pub(crate) position_attribute: u32,
    pub(crate) uv_attribute: u32,
    pub(crate) sampler_uniform: WebGlUniformLocation,
}

pub fn create_shader(context: &mut WebGl2RenderingContext) -> Result<Shader, String> {
    let vert_shader = compile_shader(
        &context,
        WebGl2RenderingContext::VERTEX_SHADER,
        r#"
        attribute vec4 aPosition;
        attribute vec2 aUv;
        
        varying highp vec2 vUv;

        void main() {
            gl_Position = aPosition;
            vUv = aUv;
        }
    "#,
    )?;
    let frag_shader = compile_shader(
        &context,
        WebGl2RenderingContext::FRAGMENT_SHADER,
        r#"
        varying highp vec2 vUv;

        uniform sampler2D uSampler;

        void main() {
            gl_FragColor = texture2D(uSampler, vUv);
        }
    "#,
    )?;
    let program = link_program(&context, &vert_shader, &frag_shader)?;

    let position_attribute = context.get_attrib_location(&program, "aPosition");
    if position_attribute < 0 {
        panic!("position attribute not found!");
    }
    
    let uv_attribute = context.get_attrib_location(&program, "aUv");
    if uv_attribute < 0 {
        panic!("uv attribute not found!");
    }

    let sampler_uniform = context.get_uniform_location(&program, "uSampler").unwrap();


    Ok(Shader {
        program,
        position_attribute: position_attribute as u32,
        uv_attribute: uv_attribute as u32,
        sampler_uniform,
    })
}

fn compile_shader(
    context: &WebGl2RenderingContext,
    shader_type: u32,
    source: &str,
) -> Result<WebGlShader, String> {
    let shader = context
        .create_shader(shader_type)
        .ok_or_else(|| String::from("Unable to create shader object"))?;
    context.shader_source(&shader, source);
    context.compile_shader(&shader);

    if context
        .get_shader_parameter(&shader, WebGl2RenderingContext::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(shader)
    } else {
        Err(context
            .get_shader_info_log(&shader)
            .unwrap_or_else(|| String::from("Unknown error creating shader")))
    }
}

fn link_program(
    context: &WebGl2RenderingContext,
    vert_shader: &WebGlShader,
    frag_shader: &WebGlShader,
) -> Result<WebGlProgram, String> {
    let program = context
        .create_program()
        .ok_or_else(|| String::from("Unable to create shader object"))?;

    context.attach_shader(&program, vert_shader);
    context.attach_shader(&program, frag_shader);

    context.link_program(&program);

    if context
        .get_program_parameter(&program, WebGl2RenderingContext::LINK_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(program)
    } else {
        Err(context
            .get_program_info_log(&program)
            .unwrap_or_else(|| String::from("Unknown error creating program object")))
    }
}
