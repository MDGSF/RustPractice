use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    explain("empty", &map);

    map.insert('a', 1);
    explain("added 1", &map);

    map.insert('b', 2);
    map.insert('c', 3);
    explain("added 3", &map);

    map.insert('d', 4);
    explain("added 4", &map);

    assert_eq!(map.get(&'a'), Some(&1));
    assert_eq!(map.get_key_value(&'b'), Some((&'b', &2)));

    map.remove(&'a');
    assert!(!map.contains_key(&'a'));
    assert_eq!(map.get(&'a'), None);
    explain("removed", &map);

    map.shrink_to_fit();
    explain("shrinked", &map);
}

fn explain<K, V>(name: &str, map: &HashMap<K, V>) {
    println!("{}: len: {}, cap: {}", name, map.len(), map.capacity());
}
