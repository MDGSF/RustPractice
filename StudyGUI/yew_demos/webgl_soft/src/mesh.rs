use crate::model::Model;
use std::sync::Arc;
use wasm_bindgen::JsValue;
use web_sys::{
    Blob, HtmlImageElement, Url, WebGl2RenderingContext as GL, WebGlTexture,
    WebGlVertexArrayObject,
};

lazy_static! {
    static ref TEXTURE_MIN_FILTERS: Vec<u32> = {
        let mut v = Vec::new();
        v.push(GL::NEAREST);
        v.push(GL::LINEAR);
        v.push(GL::NEAREST_MIPMAP_LINEAR);
        v.push(GL::LINEAR_MIPMAP_LINEAR);
        v
    };
}

pub struct Mesh {
    gl: Arc<GL>,
    #[allow(unused_variables)]
    resize_quality: usize,
    pub model: Model,
    pub color_texture: Option<WebGlTexture>,
    pub vao: Option<WebGlVertexArrayObject>,
}

impl Mesh {
    pub fn new(gl: Arc<GL>, model: Model, _image_data: String) -> Self {
        let vao = gl.create_vertex_array();
        let mesh = Self {
            gl,
            resize_quality: 0,
            model,
            color_texture: None,
            vao,
        };
        // mesh.create_texture(image_data);
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

    #[allow(dead_code)]
    fn create_texture(&mut self, image_data: String) {
        let texture = self.gl.create_texture();
        self.gl.bind_texture(GL::TEXTURE_2D, texture.as_ref());

        self.gl.pixel_storei(GL::UNPACK_ALIGNMENT, 1);
        self.gl.pixel_storei(GL::UNPACK_FLIP_Y_WEBGL, 1);

        self.gl.tex_parameteri(
            GL::TEXTURE_2D,
            GL::TEXTURE_MIN_FILTER,
            TEXTURE_MIN_FILTERS[self.resize_quality] as i32,
        );
        self.gl.tex_parameteri(
            GL::TEXTURE_2D,
            GL::TEXTURE_MAG_FILTER,
            GL::LINEAR as i32,
        );

        let image_data_js_value = JsValue::from_str(&image_data);
        let blob =
            Blob::new_with_u8_array_sequence(&image_data_js_value).unwrap();
        let url = Url::create_object_url_with_blob(&blob).unwrap();
        let image = HtmlImageElement::new().unwrap();
        image.set_src(&url);
        let image_width = image.natural_width();
        let image_height = image.natural_height();

        let level = 0;
        let internalformat = GL::RGBA;
        let border = 0;
        let format = GL::RGBA;
        let type_ = GL::UNSIGNED_BYTE;
        self.gl.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_html_image_element(
            GL::TEXTURE_2D,
            level,
            internalformat as i32,
            image_width as i32,
            image_height as i32,
            border,
            format,
            type_,
            &image,
        ).unwrap();

        if self.resize_quality >= 2 {
            self.gl.generate_mipmap(GL::TEXTURE_2D);
        }

        self.color_texture = texture;
    }
}
