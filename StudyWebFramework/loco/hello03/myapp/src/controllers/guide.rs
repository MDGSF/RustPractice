#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use axum::debug_handler;
use loco_rs::prelude::*;

#[debug_handler]
pub async fn index(State(_ctx): State<AppContext>) -> Result<Response> {
    // format::empty()
    format::text("hello")
}

pub fn routes() -> Routes {
    Routes::new().prefix("api/guides/").add("/", get(index))
}
