use anyhow::{anyhow, Result};
use web_sys::{WebGl2RenderingContext as GL, WebGlProgram, WebGlShader};

/// - @brief: 创建指定类型的 shader 着色器
/// - @param gl: webgl handle
/// - @param shader_type_str: 取值为 "vertex" 或 "fragment"
/// - @param shader_source: glsl 代码
/// - @return: 返回 shader 着色器句柄
pub fn create_shader(
    gl: &GL,
    shader_type_str: &str,
    shader_source: &str,
) -> Result<WebGlShader> {
    let shader_type = match shader_type_str {
        "vertex" => GL::VERTEX_SHADER,
        "fragment" => GL::FRAGMENT_SHADER,
        _ => {
            return Err(anyhow!(format!(
                "invalid shader type {}",
                shader_type_str
            )));
        }
    };

    let shader = gl.create_shader(shader_type).unwrap();
    gl.shader_source(&shader, shader_source);
    gl.compile_shader(&shader);

    if gl
        .get_shader_parameter(&shader, GL::COMPILE_STATUS)
        .is_falsy()
    {
        let message = gl.get_shader_info_log(&shader);
        return Err(anyhow!(format!(
            "cannot create GLSL {} shader: {:?}",
            shader_type_str, message
        )));
    }

    Ok(shader)
}

/// - @brief: 创建 glsl 程序
/// - @param gl: webgl handle
/// - @param vert_shader_source: vertex shader 对应的 glsl 代码
/// - @param frag_shader_source: fragment shader 对应的 glsl 代码
/// - @return: glsl program handle
pub fn create_program(
    gl: &GL,
    vert_shader_source: &str,
    frag_shader_source: &str,
) -> Result<WebGlProgram> {
    let vert_shader = create_shader(gl, "vertex", vert_shader_source)?;
    let frag_shader = create_shader(gl, "fragment", frag_shader_source)?;

    let program = gl.create_program().unwrap();
    gl.attach_shader(&program, &vert_shader);
    gl.attach_shader(&program, &frag_shader);
    gl.link_program(&program);

    let link_ok = gl.get_program_parameter(&program, GL::LINK_STATUS);
    if link_ok.is_falsy() {
        let message = gl.get_program_info_log(&program);
        return Err(anyhow!(format!(
            "cannot link GLSL program: {:?}",
            message
        )));
    }

    gl.delete_shader(Some(&vert_shader));
    gl.delete_shader(Some(&frag_shader));

    Ok(program)
}
