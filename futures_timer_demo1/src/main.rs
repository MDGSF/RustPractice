use futures_timer::Delay;
use std::time::Duration;

#[async_std::main]
async fn main() {
    let now = Delay::new(Duration::from_secs(3)).await;
    println!("waited for 3 secs");
}
