use actix_web::{HttpRequest, HttpResponse};
use serde::Serialize;
use tera::{Context, Tera};

#[derive(Serialize)]
struct IndexData {
    working_directory: String,
}

pub async fn index(_req: HttpRequest) -> HttpResponse {
    let data = IndexData {
        working_directory: std::env::current_dir()
            .unwrap()
            .to_str()
            .unwrap()
            .to_owned(),
    };
    let v = serde_json::to_value(data).unwrap();
    let context = Context::from_value(v).unwrap();

    let mut tpl: Tera = Default::default();
    let rendered = tpl
        .render_str(include_str!("../static/index.html"), &context)
        .unwrap();

    HttpResponse::Ok().body(rendered)
}
