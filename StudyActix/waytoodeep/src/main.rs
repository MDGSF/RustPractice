use color_eyre::Report;
use futures::{stream::FuturesUnordered, StreamExt};
use reqwest::Client;
use std::future::Future;
use std::sync::Arc;
use tokio_rustls::{rustls::ClientConfig, TlsConnector};
use tracing::info;
use tracing_subscriber::EnvFilter;
use webpki::DNSNameRef;

pub const URL_1: &str = "https://fasterthanli.me/articles/whats-in-the-box";
pub const URL_2: &str = "https://fasterthanli.me/series/advent-of-code-2020/part-13";

fn _fetch_thing_raw<'a>(
  client: &'a Client,
  url: &'a str,
) -> impl Future<Output = Result<(), Report>> + 'a {
  async move {
    let res = client.get(url).send().await?.error_for_status()?;
    info!(%url, content_type = ?res.headers().get("content-type"), "Got a response!");
    Ok(())
  }
}

async fn fetch_thing(client: Client, url: &str) -> Result<(), Report> {
  let res = client.get(url).send().await?.error_for_status()?;
  info!(%url, content_type = ?res.headers().get("content-type"), "Got a response!");
  Ok(())
}

async fn fetch_thing2(name: &str) -> Result<(), Report> {
  Ok(())
}

fn _type_name_of<T>(_: &T) -> &'static str {
  std::any::type_name::<T>()
}

fn setup() -> Result<(), Report> {
  if std::env::var("RUST_LIB_BACKTRACE").is_err() {
    std::env::set_var("RUST_LIB_BACKTRACE", "1")
  }
  color_eyre::install()?;

  if std::env::var("RUST_LOG").is_err() {
    std::env::set_var("RUST_LOG", "info")
  }
  tracing_subscriber::fmt::fmt()
    .with_env_filter(EnvFilter::from_default_env())
    .init();

  Ok(())
}

async fn test1() -> Result<(), Report> {
  let client = Client::new();

  let mut group = vec![
    fetch_thing(client.clone(), URL_1),
    fetch_thing(client, URL_2),
  ]
  .into_iter()
  .collect::<FuturesUnordered<_>>();

  while let Some(item) = group.next().await {
    item?;
  }

  Ok(())
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Report> {
  setup()?;

  test1().await?;

  Ok(())
}
