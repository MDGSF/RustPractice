#[async_std::main]
async fn main() -> Result<(), std::io::Error> {
  let mut app = tide::new();
  app.at("/").get(|_| async move { "Hello, World!" });
  app.listen("127.0.0.1:8080").await
}
