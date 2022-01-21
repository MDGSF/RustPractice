use crate::glsl;
use crate::mesh;
use web_sys::{
    WebGl2RenderingContext as GL, WebGlProgram, WebGlUniformLocation,
};

pub struct Renderer<'a> {
    gl: &'a GL,
    program: WebGlProgram,
    p: Option<WebGlUniformLocation>,
    v: Option<WebGlUniformLocation>,
}

impl<'a> Renderer<'a> {
    pub fn new(
        gl: &'a GL,
        vert_shader_source: &'a str,
        frag_shader_source: &'a str,
    ) -> Self {
        // init webgl
        gl.enable(GL::DEPTH_TEST);
        gl.depth_func(GL::LEQUAL);
        gl.clear_color(0.0, 0.0, 0.0, 1.0);
        gl.clear_depth(1.0);

        // init shader program
        let program =
            glsl::create_program(gl, vert_shader_source, frag_shader_source)
                .unwrap();
        let p = gl.get_uniform_location(&program, "P");
        let v = gl.get_uniform_location(&program, "V");

        Self { gl, program, p, v }
    }

    pub fn draw(
        &self,
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

impl<'a> Drop for Renderer<'a> {
    fn drop(&mut self) {
        self.gl.delete_program(Some(&self.program));
    }
}
