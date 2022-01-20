use crate::model::Model;
use web_sys::{WebGl2RenderingContext as GL, WebGlVertexArrayObject};

pub struct Mesh<'a> {
    gl: &'a GL,
    model: Model,
    vao: Option<WebGlVertexArrayObject>,
}

impl<'a> Mesh<'a> {
    pub fn new(gl: &'a GL, model: Model) -> Self {
        let mesh = Self {
            gl,
            model,
            vao: gl.create_vertex_array(),
        };
        mesh.upload_data();
        mesh
    }

    fn upload_data(&self) {
        self.gl.bind_vertex_array(self.vao.as_ref());
        self.gl.enable_vertex_attrib_array(0);
        self.gl.enable_vertex_attrib_array(1);

        // position
        let vertex_buffer = self.gl.create_buffer().unwrap();
        self.gl.bind_buffer(GL::ARRAY_BUFFER, Some(&vertex_buffer));
        let verts = js_sys::Float32Array::from(self.model.vertices.as_slice());
        self.gl.buffer_data_with_array_buffer_view(
            GL::ARRAY_BUFFER,
            &verts,
            GL::STATIC_DRAW,
        );
        self.gl
            .vertex_attrib_pointer_with_i32(0, 3, GL::FLOAT, false, 0, 0);

        // color
        let color_buffer = self.gl.create_buffer().unwrap();
        self.gl.bind_buffer(GL::ARRAY_BUFFER, Some(&color_buffer));
        let colors = js_sys::Float32Array::from(self.model.colors.as_slice());
        self.gl.buffer_data_with_array_buffer_view(
            GL::ARRAY_BUFFER,
            &colors,
            GL::STATIC_DRAW,
        );
        self.gl
            .vertex_attrib_pointer_with_i32(1, 3, GL::FLOAT, false, 0, 0);

        // Create and store data into index buffer
        let index_buffer = self.gl.create_buffer().unwrap();
        self.gl
            .bind_buffer(GL::ELEMENT_ARRAY_BUFFER, Some(&index_buffer));
        let indices = js_sys::Uint16Array::from(self.model.indices.as_slice());
        self.gl.buffer_data_with_array_buffer_view(
            GL::ELEMENT_ARRAY_BUFFER,
            &indices,
            GL::STATIC_DRAW,
        );

        self.gl.bind_vertex_array(None);
    }
}
