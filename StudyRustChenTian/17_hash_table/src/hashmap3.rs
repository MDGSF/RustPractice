use std::collections::HashMap;

fn main() {
    let map = HashMap::new();
    let mut map = explain("empty", map);

    map.insert(String::from('a'), 1);
    let mut map = explain("added 1", map);

    map.insert(String::from('b'), 2);
    map.insert(String::from('c'), 3);
    let mut map = explain("added 3", map);

    map.insert(String::from('d'), 4);
    let mut map = explain("added 3", map);

    map.remove("a");
    explain("final", map);
}

fn explain<K, V>(name: &str, map: HashMap<K, V>) -> HashMap<K, V> {
    let arr: [usize; 6] = unsafe { std::mem::transmute(map) };
    println!(
        "{}: bucket_mask 0x{:x}, ctrl 0x{:x}, growth_left: {}, items: {}",
        name, arr[2], arr[3], arr[4], arr[5],
    );
    unsafe { std::mem::transmute(arr) }
}
