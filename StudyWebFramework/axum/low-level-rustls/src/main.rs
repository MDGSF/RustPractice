use axum::{handler::get, Router};
use hyper::server::conn::Http;
use std::{fs::File, io::BufReader, sync::Arc};
use tokio::net::TcpListener;
use tokio_rustls::{
  rustls::{
    internal::pemfile::certs, internal::pemfile::pkcs8_private_keys,
    NoClientAuth, ServerConfig,
  },
  TlsAcceptor,
};

#[tokio::main]
async fn main() {
  if std::env::var("RUST_LOG").is_err() {
    std::env::set_var("RUST_LOG", "low_level_rustls=debug");
  }
  tracing_subscriber::fmt::init();

  let rustls_config = rustls_server_config(
    "self_signed_certs/key.pem",
    "self_signed_certs/cert.pem",
  );

  let acceptor = TlsAcceptor::from(rustls_config);
  let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();

  let app = Router::new().route("/", get(handler));

  loop {
    let (stream, _addr) = listener.accept().await.unwrap();
    let acceptor = acceptor.clone();
    let app = app.clone();

    tokio::spawn(async move {
      if let Ok(stream) = acceptor.accept(stream).await {
        let _ = Http::new().serve_connection(stream, app).await;
      }
    });
  }
}

async fn handler() -> &'static str {
  "Hello, World!"
}

fn rustls_server_config(key: &str, cert: &str) -> Arc<ServerConfig> {
  let mut key_reader = BufReader::new(File::open(key).unwrap());
  let mut cert_reader = BufReader::new(File::open(cert).unwrap());

  let key = pkcs8_private_keys(&mut key_reader).unwrap().remove(0);
  let certs = certs(&mut cert_reader).unwrap();

  let mut config = ServerConfig::new(NoClientAuth::new());
  config.set_single_cert(certs, key).unwrap();
  config.set_protocols(&[b"h2".to_vec(), b"http/1.1".to_vec()]);
  Arc::new(config)
}
