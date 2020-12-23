use std::collections::HashMap;
use std::hash::Hash;

fn get_default<'r, K: Hash + Eq + Copy, V: Default>(
    map: &'r mut HashMap<K, V>,
    key: K,
) -> &'r mut V {
    if map.contains_key(&key) {
        return match map.get_mut(&key) {
            Some(value) => value,
            None => unreachable!(),
        };
    }

    map.insert(key, V::default());
    map.get_mut(&key).unwrap()
}

fn main() {
    println!("Hello, world!");
}
