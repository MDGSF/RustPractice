extern crate crossbeam;

use crossbeam::thread::scope;

fn main() {
    let array = [1, 2, 3];
    scope(|scope| {
        for i in &array {
            scope.spawn(move || {
                println!("element: {}", i);
            });
        }
    });
}
