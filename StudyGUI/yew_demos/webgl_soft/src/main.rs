use gloo_render::{request_animation_frame, AnimationFrame};
use std::sync::Arc;
use wasm_bindgen::JsCast;
use web_sys::{HtmlCanvasElement, WebGl2RenderingContext as GL};
use webgl2::mesh;
use webgl2::model;
use webgl2::renderer;
use yew::html::Scope;
use yew::{html, Component, Context, Html, NodeRef};

pub enum Msg {
    Render(f64),
}

pub struct App {
    gl: Option<Arc<GL>>,
    node_ref: NodeRef,
    _render_loop: Option<AnimationFrame>,
    renderer: Option<renderer::Renderer>,
    mesh: Option<mesh::Mesh>,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            gl: None,
            node_ref: NodeRef::default(),
            _render_loop: None,
            renderer: None,
            mesh: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Render(timestamp) => {
                // Render functions are likely to get quite large, so it is good practice to split
                // it into it's own function rather than keeping it inline in the update match
                // case. This also allows for updating other UI elements that may be rendered in
                // the DOM like a framerate counter, or other overlaid textual elements.
                self.render_gl(timestamp, ctx.link());
                false
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <canvas ref={self.node_ref.clone()} />
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render {
            let canvas = self.node_ref.cast::<HtmlCanvasElement>().unwrap();

            let gl: GL = canvas
                .get_context("webgl2")
                .unwrap()
                .unwrap()
                .dyn_into()
                .unwrap();
            let gl = Arc::new(gl);
            self.gl = Some(gl.clone());

            let model = model::Model::new_from_json(model::MODEL_CUBE);
            self.mesh = Some(mesh::Mesh::new(gl.clone(), model));

            let vert_code = include_str!("../glsl/compiled_vert.glsl");
            let frag_code = include_str!("../glsl/compiled_frag.glsl");
            self.renderer =
                Some(renderer::Renderer::new(gl, vert_code, frag_code));

            // The callback to request animation frame is passed a time value which can be used for
            // rendering motion independent of the framerate which may vary.
            let handle = {
                let link = ctx.link().clone();
                request_animation_frame(move |time| {
                    link.send_message(Msg::Render(time))
                })
            };

            // A reference to the handle must be stored, otherwise it is dropped and the render won't
            // occur.
            self._render_loop = Some(handle);
        }
    }
}

impl App {
    fn render_gl(&mut self, _timestamp: f64, link: &Scope<Self>) {
        let mesh = self.mesh.as_ref().expect("mesh not initialized");
        let renderer =
            self.renderer.as_ref().expect("renderer not initialized");

        let view_matrix = vec![];
        let proj_matrix = vec![];

        renderer.draw(mesh, &view_matrix, &proj_matrix);

        let handle = {
            let link = link.clone();
            request_animation_frame(move |time| {
                link.send_message(Msg::Render(time))
            })
        };

        // A reference to the new handle must be retained for the next render to run.
        self._render_loop = Some(handle);
    }
}

fn main() {
    yew::start_app::<App>();
}
