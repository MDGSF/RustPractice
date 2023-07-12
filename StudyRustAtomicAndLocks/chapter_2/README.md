# chapter 2

## fetch

```rust
impl AtomicI32 {
  pub fn fetch_add(&self, v: i32, ordering: Ordering) -> i32;
  pub fn fetch_sub(&self, v: i32, ordering: Ordering) -> i32;
  pub fn fetch_or(&self, v: i32, ordering: Ordering) -> i32;
  pub fn fetch_and(&self, v: i32, ordering: Ordering) -> i32;
  pub fn fetch_nand(&self, v: i32, ordering: Ordering) -> i32;
  pub fn fetch_xor(&self, v: i32, ordering: Ordering) -> i32;
  pub fn fetch_max(&self, v: i32, ordering: Ordering) -> i32;
  pub fn fetch_min(&self, v: i32, ordering: Ordering) -> i32;
  pub fn swap(&self, v: i32, ordering: Ordering) -> i32; // "fetch_store"
}
```

## compare and exchange

```rust
impl AtomicI32 {
  pub fn compare_exchange(
      &self,
      expected: i32,
      new: i32,
      success_order: Ordering,
      failure_order: Ordering
      ) -> Result<i32, i32>;
}

impl AtomicI32 {
    pub fn compare_exchange(&self, expected: i32, new: i32) -> Result<i32, i32> {
        // In reality, the load, comparison and store,
        // all happen as a single atomic operation.
        let v = self.load();
        if v == expected {
            // Value is as expected.
            // Replace it and report success.
            self.store(new);
            Ok(v)
        } else {
            // The value was not as expected.
            // Leave it untouched and report failure.
            Err(v)
        }
    }
}
```

## compare exchange weak

Next to `compare_exchange`, there is a similar method named `compare_exchange_weak`. The difference is that the weak version may still
sometimes leave the value untouched and return an Err, even though the atomic value matched the expected value.

