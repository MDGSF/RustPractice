use dashmap::DashMap;

fn main() {
    let reviews = DashMap::new();

    for i in 0..100 {
        reviews.insert(i, i);
        println!(
            "i = {}, capaticy = {}, shards.len = {}",
            i,
            reviews.capacity(),
            reviews.shards().len()
        );
    }
}
