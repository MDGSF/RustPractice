use anyhow::{anyhow, Result};
use clap::Parser;
use colored::Colorize;
use mime::Mime;
use reqwest::{header, Client, Response, Url};
use std::{collections::HashMap, str::FromStr};
use syntect::{
  easy::HighlightLines,
  highlighting::{Style, ThemeSet},
  parsing::SyntaxSet,
  util::{as_24_bit_terminal_escaped, LinesWithEndings},
};

/// A naive httpie implementation with Rust.
#[derive(Parser, Debug)]
#[clap(version = "1.0", author = "HuangJian")]
struct Opts {
  #[clap(subcommand)]
  subcmd: SubCommand,
}

#[derive(Parser, Debug)]
enum SubCommand {
  Get(Get),
  Post(Post),
}

/// feed get with an url an we will retrieve the response for you
#[derive(Parser, Debug)]
struct Get {
  /// HTTP 请求的 URL
  #[clap(parse(try_from_str = parse_url))]
  url: String,
}

/// feed post with an url and optional key=val pairs. We will post the data
/// as JSON, and retrieve the response for you
#[derive(Parser, Debug)]
struct Post {
  /// HTTP 请求的 URL
  #[clap(parse(try_from_str = parse_url))]
  url: String,

  /// HTTP 请求的 body
  #[clap(parse(try_from_str = parse_kv_pair))]
  body: Vec<KvPair>,
}

/// 命令行中的 key=val 可以通过 parse_kv_pair 解析成 KvPair 结构
#[derive(Debug, PartialEq)]
struct KvPair {
  k: String,
  v: String,
}

impl FromStr for KvPair {
  type Err = anyhow::Error;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut split = s.split('=');
    let err = || anyhow!(format!("Failed to parse {}", s));
    Ok(Self {
      k: (split.next().ok_or_else(err)?).to_string(),
      v: (split.next().ok_or_else(err)?).to_string(),
    })
  }
}

fn parse_kv_pair(s: &str) -> Result<KvPair> {
  s.parse()
}

fn parse_url(s: &str) -> Result<String> {
  // 这里仅仅检查下 URL 是否合法
  let _url: Url = s.parse()?;

  Ok(s.into())
}

// 处理 get 子命令
async fn get(client: Client, args: &Get) -> Result<()> {
  let resp = client.get(&args.url).send().await?;
  Ok(print_resp(resp).await?)
}

// 处理 post 子命令
async fn post(client: Client, args: &Post) -> Result<()> {
  let mut body = HashMap::new();
  for pair in args.body.iter() {
    body.insert(&pair.k, &pair.v);
  }
  let resp = client.post(&args.url).json(&body).send().await?;
  Ok(print_resp(resp).await?)
}

fn print_status(resp: &Response) {
  let status = format!("{:?} {}", resp.version(), resp.status()).blue();
  println!("{}\n", status);
}

fn print_headers(resp: &Response) {
  for (name, value) in resp.headers() {
    println!("{}: {:?}", name.to_string().green(), value);
  }
  println!();
}

fn print_body(m: Option<Mime>, body: &str) {
  match m {
    Some(v) if v == mime::APPLICATION_JSON => print_syntect(body, "json"),
    Some(v) if v == mime::TEXT_HTML => print_syntect(body, "html"),
    _ => println!("{}", body),
  }
}

fn print_syntect(s: &str, ext: &str) {
  // Load these once at the start of your program
  let ps = SyntaxSet::load_defaults_newlines();
  let ts = ThemeSet::load_defaults();
  let syntax = ps.find_syntax_by_extension(ext).unwrap();
  let mut h = HighlightLines::new(syntax, &ts.themes["base16-ocean.dark"]);
  for line in LinesWithEndings::from(s) {
    let ranges: Vec<(Style, &str)> = h.highlight(line, &ps);
    let escaped = as_24_bit_terminal_escaped(&ranges[..], true);
    print!("{}", escaped);
  }
}

async fn print_resp(resp: Response) -> Result<()> {
  print_status(&resp);
  print_headers(&resp);
  let mime = get_content_type(&resp);
  let body = resp.text().await?;
  print_body(mime, &body);
  Ok(())
}

fn get_content_type(resp: &Response) -> Option<Mime> {
  resp
    .headers()
    .get(header::CONTENT_TYPE)
    .map(|v| v.to_str().unwrap().parse().unwrap())
}

#[tokio::main]
async fn main() -> Result<()> {
  let opts: Opts = Opts::parse();
  let mut headers = header::HeaderMap::new();
  headers.insert("X-POWERED-BY", "Rust".parse()?);
  headers.insert(header::USER_AGENT, "Rust Httpie".parse()?);
  let client = reqwest::Client::builder()
    .default_headers(headers)
    .build()?;
  let result = match opts.subcmd {
    SubCommand::Get(ref args) => get(client, args).await?,
    SubCommand::Post(ref args) => post(client, args).await?,
  };
  Ok(result)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn parse_url_works() {
    assert!(parse_url("abc").is_err());
    assert!(parse_url("http://abc.xyz").is_ok());
    assert!(parse_url("https://httpbin.org/post").is_ok());
  }

  #[test]
  fn parse_kv_pair_works() {
    assert!(parse_kv_pair("a").is_err());
    assert_eq!(
      parse_kv_pair("a=1").unwrap(),
      KvPair {
        k: "a".into(),
        v: "1".into(),
      }
    );

    assert_eq!(
      parse_kv_pair("b=").unwrap(),
      KvPair {
        k: "b".into(),
        v: "".into(),
      }
    );
  }
}
