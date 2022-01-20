use web_sys::WebGl2RenderingContext as GL;

pub struct Renderer<'a> {
    gl: &'a GL,
}

impl<'a> Renderer<'a> {
    pub fn new(gl: &'a GL) -> Self {
        Self { gl }
    }
}
