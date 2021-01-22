fn main() {
  awak::block_on(async {
    let handle = awak::spawn(async {
      println!("Running task...");
      1 + 2
    });

    assert_eq!(handle.await, 3);
  });
}