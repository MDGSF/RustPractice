use crate::mesh;
use std::sync::Arc;
use wasm_bindgen::JsValue;
use web_sys::CanvasRenderingContext2d;

pub struct RendererSoft {
    ctx_soft: Arc<CanvasRenderingContext2d>,
    width: f32,
    height: f32,
}

impl RendererSoft {
    pub fn new(
        ctx_soft: Arc<CanvasRenderingContext2d>,
        width: f32,
        height: f32,
    ) -> Self {
        Self {
            ctx_soft,
            width,
            height,
        }
    }

    pub fn draw_scene(
        &self,
        scene: &[&mesh::Mesh],
        mat_model: &na::Matrix4<f32>,
        mat_view: &na::Matrix4<f32>,
        mat_projection: &na::Matrix4<f32>,
    ) {
        // 清屏
        self.ctx_soft.set_fill_style(&JsValue::from_str("#000"));
        self.ctx_soft.fill_rect(
            0.0,
            0.0,
            self.width as f64,
            self.height as f64,
        );

        // 文字
        self.ctx_soft.set_fill_style(&JsValue::from_str("#fff"));
        self.ctx_soft.set_font("32px monospace");
        self.ctx_soft
            .fill_text("Software Renderer", 40.0, 40.0)
            .unwrap();

        for mesh in scene.iter() {
            let index_count = mesh.model.indices.len();
            let triangle_count = index_count / 3;
            for i in 0..triangle_count {
                let mut tri = vec![];
                for j in 0..3 {
                    let k = (mesh.model.indices[i * 3 + j] * 3) as usize;
                    let x = mesh.model.vertices[k];
                    let y = mesh.model.vertices[k + 1];
                    let z = mesh.model.vertices[k + 2];
                    let point = na::Point3::new(x, y, z);
                    tri.push(point);
                }

                let color = "#f00";
                self.draw_triangle_soft(
                    tri[0],
                    tri[1],
                    tri[2],
                    mat_model,
                    mat_view,
                    mat_projection,
                    &color,
                )
            }
        }
    }

    pub fn draw_triangle_soft(
        &self,
        triangle_p0: na::Point3<f32>,
        triangle_p1: na::Point3<f32>,
        triangle_p2: na::Point3<f32>,
        mat_model: &na::Matrix4<f32>,
        mat_view: &na::Matrix4<f32>,
        mat_projection: &na::Matrix4<f32>,
        color: &str,
    ) {
        let p0 = self.transform_point(
            triangle_p0,
            mat_model,
            mat_view,
            mat_projection,
        );
        let p1 = self.transform_point(
            triangle_p1,
            mat_model,
            mat_view,
            mat_projection,
        );
        let p2 = self.transform_point(
            triangle_p2,
            mat_model,
            mat_view,
            mat_projection,
        );
        // 逆时针 = 指向+z方向 = 指向观众 = 正面
        // counter-clockwise
        let ccw = self.get_triangle_orientation(&p0, &p1, &p2);
        if ccw {
            self.draw_triangle_2d(&p0, &p1, &p2, color);
            self.draw_point(&p0, "#ff0");
            self.draw_point(&p1, "#ff0");
            self.draw_point(&p2, "#ff0");
        }
    }

    /// 对点 point 进行一系列坐标系转换
    fn transform_point(
        &self,
        point: na::Point3<f32>,
        mat_model: &na::Matrix4<f32>,
        mat_view: &na::Matrix4<f32>,
        mat_projection: &na::Matrix4<f32>,
    ) -> na::Point2<f32> {
        // 模型坐标系
        let v = na::Vector4::new(point.x, point.y, point.z, 1.0);

        // 世界坐标系: M * point
        let world = mat_model * v;

        // View/camera Space: V * M * point
        let view = mat_view * world;

        // Clipping Space: P * V * M * point
        let mut clip = mat_projection * view;

        // clip: [x, y, z, w] 齐次坐标

        // 除以 w
        clip.x /= clip.w; // x/w
        clip.y /= clip.w; // y/w
        clip.z /= clip.w; // z/w  深度缓冲区
        clip.w /= clip.w; // 1.0  丢弃

        // NDC坐标系的x, y, 取值范围：[-1, 1]
        let xn = clip.x;
        let yn = clip.y;

        let xw = (xn + 1.0) * 0.5 * self.width; // [0, w]
        let xh = (1.0 - yn) * 0.5 * self.height; // [h, 0]

        na::Point2::new(xw, xh)
    }

    /// color: "#000"
    fn draw_triangle_2d(
        &self,
        p0: &na::Point2<f32>,
        p1: &na::Point2<f32>,
        p2: &na::Point2<f32>,
        color: &str,
    ) {
        self.draw_triangle_2d_f64(
            na::Point2::new(p0.x as f64, p0.y as f64),
            na::Point2::new(p1.x as f64, p1.y as f64),
            na::Point2::new(p2.x as f64, p2.y as f64),
            color,
        )
    }

    /// color: "#000"
    fn draw_triangle_2d_f64(
        &self,
        p0: na::Point2<f64>,
        p1: na::Point2<f64>,
        p2: na::Point2<f64>,
        color: &str,
    ) {
        self.ctx_soft.set_stroke_style(&JsValue::from_str(color));
        self.ctx_soft.set_fill_style(&JsValue::from_str(color));

        self.ctx_soft.begin_path();

        self.ctx_soft.move_to(p0.x, p0.y);
        self.ctx_soft.line_to(p1.x, p1.y);
        self.ctx_soft.line_to(p2.x, p2.y);

        self.ctx_soft.close_path();
        self.ctx_soft.stroke();
        self.ctx_soft.fill();
    }

    /// color: "#000"
    fn draw_point(&self, point: &na::Point2<f32>, color: &str) {
        self.draw_point_f64(
            &na::Point2::new(point.x as f64, point.y as f64),
            color,
        );
    }

    /// color: "#000"
    fn draw_point_f64(&self, point: &na::Point2<f64>, color: &str) {
        let r: f64 = 2.0;
        self.ctx_soft.set_fill_style(&JsValue::from_str(color));
        self.ctx_soft
            .fill_rect(point.x - r, point.y - r, r * 2.0, r * 2.0);
    }

    fn get_triangle_orientation(
        &self,
        p0: &na::Point2<f32>,
        p1: &na::Point2<f32>,
        p2: &na::Point2<f32>,
    ) -> bool {
        let v0 = na::Point3::new(p0.x, p0.y, 1.0);
        let v1 = na::Point3::new(p1.x, p1.y, 1.0);
        let v2 = na::Point3::new(p2.x, p2.y, 1.0);

        let e1 = v1 - v0; // 向量 e1
        let e2 = v2 - v1; // 向量 e2
        let x = e1.cross(&e2); // 向量叉乘

        x.z > 0.0
    }
}
