pub mod async_lib;
pub mod sync_lib;

pub use sync_lib::*;

#[cfg(test)]
mod tests {
  #[test]
  fn it_works() {
    assert_eq!(2 + 2, 4);
  }
}
