pub mod moption;
pub use moption::*;

pub mod mresult;
pub use mresult::*;

#[cfg(test)]
mod tests {
  #[test]
  fn it_works() {
    assert_eq!(2 + 2, 4);
  }
}
