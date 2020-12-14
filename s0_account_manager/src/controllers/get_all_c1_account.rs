use tide::Request;

pub async fn get_all_c1_account(_req: Request<()>) -> tide::Result {
  Ok(format!("order_shoes").into())
}
