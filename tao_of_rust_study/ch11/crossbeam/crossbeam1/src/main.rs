use std::sync::Arc;

fn main() {
    let array = Arc::new([1, 2, 3]);
    let mut guards = vec![];

    for i in 0..array.len() {
        let a = array.clone();

        let guard = std::thread::spawn(move || {
            println!("element: {}", a[i]);
        });
        guards.push(guard);
    }

    for guard in guards {
        guard.join().unwrap();
    }
}
