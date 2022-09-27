use actix_web::{middleware, web, App, HttpServer};

mod hello;
mod upload_file;
mod wsserver;

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    //std::env::set_var("RUST_LOG", "sn=info;actix_web=info");
    //RUST_LOG=sn=info;actix=info cargo run

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    log::info!("starting HTTP server at http://0.0.0.0:8080");

    std::fs::create_dir_all("./tmp")?;

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(web::resource("/").to(hello::index))
            .service(hello::greet)
            .service(
                web::resource("/upload")
                    .route(web::get().to(upload_file::view))
                    .route(web::post().to(upload_file::upload_file)),
            )
            .service(web::resource("/wsview").to(wsserver::view))
            .service(web::resource("/ws").route(web::get().to(wsserver::echo_ws)))
    })
    .workers(4)
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
