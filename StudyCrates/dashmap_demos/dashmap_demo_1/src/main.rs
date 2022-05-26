use dashmap::DashMap;

fn main() {
    let reviews = DashMap::new();
    reviews.insert("Veloren", "What a fantastic game!");

    println!("m = {:?}", reviews);
}
