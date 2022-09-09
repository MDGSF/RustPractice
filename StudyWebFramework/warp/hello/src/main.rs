#![deny(warnings)]
use warp::Filter;

#[tokio::main]
async fn main() {
    let routes = warp::any().map(|| "Hello, World!");

    warp::serve(routes).run(([127, 0, 0, 1], 3000)).await;
}
