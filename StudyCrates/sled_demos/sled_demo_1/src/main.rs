fn main() {
  let tree = sled::open("welcome-to-sled").expect("open");

  // insert and get, similar to std's BTreeMap
  tree.insert("KEY1", "VAL1");
  assert_eq!(tree.get(&"KEY1"), Ok(Some(sled::IVec::from("VAL1"))));

  // range queries
  for kv in tree.range("KEY1".."KEY9") {}

  // deletion
  tree.remove(&"KEY1");

  // atomic compare and swap
  tree.compare_and_swap("KEY1", Some("VAL1"), Some("VAL2"));

  // block until all operations are stable on disk
  // (flush_async also available to get a Future)
  tree.flush();
}
