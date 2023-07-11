# chapter 2

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
