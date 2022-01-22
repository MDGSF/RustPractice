extern crate nalgebra as na;
use gloo_render::{request_animation_frame, AnimationFrame};
use na::{Isometry3, Perspective3, Point3, Rotation3, Vector3};
use std::sync::Arc;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::{
    CanvasRenderingContext2d, HtmlCanvasElement, WebGl2RenderingContext as GL,
};
use webgl2::mesh;
use webgl2::model;
use webgl2::renderer;
use webgl2::renderer_soft;
use yew::html::Scope;
use yew::{html, Component, Context, Html, NodeRef};

pub enum Msg {
    Render(f64),
}

pub struct App {
    width: f32,
    height: f32,

    ctx_soft: Option<Arc<CanvasRenderingContext2d>>,
    node_ref_soft: NodeRef,
    renderer_soft: Option<renderer_soft::RendererSoft>,

    gl: Option<Arc<GL>>,
    node_ref: NodeRef,
    renderer: Option<renderer::Renderer>,
    mesh: Option<mesh::Mesh>,
    eye: na::OPoint<f32, na::Const<3_usize>>,

    _render_loop: Option<AnimationFrame>,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            width: 400.0,
            height: 300.0,

            ctx_soft: None,
            node_ref_soft: NodeRef::default(),
            renderer_soft: None,

            gl: None,
            node_ref: NodeRef::default(),
            renderer: None,
            mesh: None,
            eye: Point3::new(0.0_f32, 0.0, 10.0),

            _render_loop: None,
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
            <div>
                <canvas id="canvas-gl"
                    width={ format!("{}", self.width) }
                    height={ format!("{}", self.height) }
                    ref={self.node_ref.clone()} />
                <canvas id="canvas-soft"
                    width={ format!("{}", self.width) }
                    height={ format!("{}", self.height) }
                    ref={self.node_ref_soft.clone()} />
            </div>
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render {
            // canvas-soft 2d init
            let canvas_soft =
                self.node_ref_soft.cast::<HtmlCanvasElement>().unwrap();
            let ctx_soft: CanvasRenderingContext2d = canvas_soft
                .get_context("2d")
                .unwrap()
                .unwrap()
                .dyn_into()
                .unwrap();
            ctx_soft.set_fill_style(&JsValue::from_str("#000"));
            ctx_soft.fill_rect(0.0, 0.0, self.width as f64, self.height as f64);
            let ctx_soft = Arc::new(ctx_soft);
            self.ctx_soft = Some(ctx_soft.clone());

            self.renderer_soft = Some(renderer_soft::RendererSoft::new(
                ctx_soft,
                self.width,
                self.height,
            ));

            // webgl init
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
        // model matrix
        let model = Isometry3::new(Vector3::x(), na::zero());
        let model_matrix = model.to_homogeneous();

        // view matrix
        let axis = Vector3::y_axis();
        let angle: f32 = 0.0001 * (180.0 / 3.14159);
        let rot = Rotation3::from_axis_angle(&axis, angle);
        self.eye = rot * self.eye;

        let target = Point3::new(0.0, 0.0, 0.0);
        let view = Isometry3::look_at_rh(&self.eye, &target, &Vector3::y());
        let view_matrix = view.to_homogeneous();
        let view_slice = view_matrix.as_slice();

        // projection matrix
        let aspect: f32 = self.width / self.height;
        let fovy: f32 = 40.0;
        let znear: f32 = 0.1;
        let zfar: f32 = 1000.0;
        let projection = Perspective3::new(aspect, fovy, znear, zfar);
        let proj_matrix = projection.as_matrix();
        let proj_slice = proj_matrix.as_slice();

        let mesh = self.mesh.as_ref().expect("mesh not initialized");

        let renderer =
            self.renderer.as_ref().expect("renderer not initialized");
        renderer.draw(mesh, &view_slice, &proj_slice);

        let mut scene = Vec::new();
        scene.push(mesh);

        let renderer_soft = self
            .renderer_soft
            .as_ref()
            .expect("renderer_soft not initialized");
        renderer_soft.draw_scene(
            scene.as_slice(),
            &model_matrix,
            &view_matrix,
            &proj_matrix,
        );

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
