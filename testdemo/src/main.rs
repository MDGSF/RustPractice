use std::time::Duration;
use std::time::SystemTime;

fn main() {
    let time = SystemTime::UNIX_EPOCH + Duration::from_secs(1);

    let time2 = Duration::new(1, 2);
    let () = time2;

    match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(n) => println!(
            "1970-01-01 00:00:00 UTC was {} seconds, {} nano ago!",
            n.as_secs(),
            n.subsec_nanos()
        ),
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    }
}
