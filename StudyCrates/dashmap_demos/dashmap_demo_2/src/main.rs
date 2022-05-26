use dashmap::DashMap;

fn main() {
    let reviews = DashMap::new();

    for i in 0..20 {
        reviews.insert(i, i);
        println!("i = {}, capaticy = {}", i, reviews.capacity());
    }
}
