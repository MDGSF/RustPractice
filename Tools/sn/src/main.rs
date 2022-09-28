use actix_web::{middleware, web, App, HttpServer};

mod exec;
mod exec1;
mod exec2;
mod exec3;
mod exec4;
mod file;
mod hello;
mod help;
mod utils;
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
            .service(web::resource("/help").to(help::help))
            .service(
                web::resource("/upload")
                    .route(web::get().to(file::view_upload))
                    .route(web::post().to(file::upload_file)),
            )
            .service(web::resource("/download").route(web::get().to(file::download_file)))
            .service(web::resource("/wsview").to(wsserver::view))
            .service(web::resource("/ws").route(web::get().to(wsserver::echo_ws)))
            .service(web::resource("/exec1").to(exec1::exec1))
            .service(web::resource("/exec2").to(exec2::exec2))
            .service(web::resource("/exec3").to(exec3::exec3))
            .service(web::resource("/exec4").to(exec4::exec4))
    })
    .workers(4)
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
