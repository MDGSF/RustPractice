use crate::glsl;
use crate::mesh;
use std::sync::Arc;
use web_sys::{
    WebGl2RenderingContext as GL, WebGlProgram, WebGlUniformLocation,
};

pub struct Renderer {
    gl: Arc<GL>,
    program: WebGlProgram,
    p: Option<WebGlUniformLocation>,
    v: Option<WebGlUniformLocation>,
    u_time: Option<WebGlUniformLocation>,
}

impl Renderer {
    pub fn new(
        gl: Arc<GL>,
        vert_shader_source: &str,
        frag_shader_source: &str,
    ) -> Self {
        // init webgl
        gl.enable(GL::DEPTH_TEST);
        gl.depth_func(GL::LEQUAL);
        gl.clear_color(0.0, 0.0, 0.0, 1.0);
        gl.clear_depth(1.0);

        // init shader program
        let program =
            glsl::create_program(&gl, vert_shader_source, frag_shader_source)
                .unwrap();
        let p = gl.get_uniform_location(&program, "P");
        let v = gl.get_uniform_location(&program, "V");
        let u_time = gl.get_uniform_location(&program, "u_time");

        Self {
            gl,
            program,
            p,
            v,
            u_time,
        }
    }

    pub fn draw(
        &self,
        timestamp: f64,
        mesh: &mesh::Mesh,
        view_matrix: &[f32],
        proj_matrix: &[f32],
    ) {
        self.gl.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT);
        self.gl.use_program(Some(&self.program));
        self.gl.uniform_matrix4fv_with_f32_array(
            self.v.as_ref(),
            false,
            view_matrix,
        );
        self.gl.uniform_matrix4fv_with_f32_array(
            self.p.as_ref(),
            false,
            proj_matrix,
        );
        self.gl.uniform1f(self.u_time.as_ref(), timestamp as f32);

        self.gl.bind_vertex_array(mesh.vao.as_ref());
        self.gl.draw_elements_with_i32(
            GL::TRIANGLES,
            mesh.model.indices.len() as i32,
            GL::UNSIGNED_SHORT,
            0,
        );
        self.gl.bind_vertex_array(None);

        let code = self.gl.get_error();
        if code != GL::NO_ERROR {
            println!("{:?}", glsl::format_gl_error(code));
        }
    }
}

impl Drop for Renderer {
    fn drop(&mut self) {
        self.gl.delete_program(Some(&self.program));
    }
}
