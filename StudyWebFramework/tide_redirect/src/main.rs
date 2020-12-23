use tide::Redirect;

#[async_std::main]
async fn main() -> tide::Result<()> {
  let mut app = tide::new();
  app.at("/").get(|_| async { Ok("meow") });
  app.at("/nori").get(Redirect::temporary("/"));
  app.listen("127.0.0.1:8080").await?;
  Ok(())
}
