use std::fmt;
use std::mem;

#[derive(Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
pub enum MOption<T> {
  Some(T),
  None,
}

impl<T> MOption<T> {
  #[inline]
  pub const fn is_some(&self) -> bool {
    matches!(*self, MOption::Some(_))
  }

  #[inline]
  pub const fn is_none(&self) -> bool {
    !self.is_some()
  }

  #[inline]
  pub fn contains<U>(&self, x: &U) -> bool
  where
    U: PartialEq<T>,
  {
    match self {
      MOption::Some(y) => x == y,
      MOption::None => false,
    }
  }

  /// &Option<T> -> Option<&T>
  #[inline]
  pub const fn as_ref(&self) -> MOption<&T> {
    match *self {
      MOption::Some(ref x) => MOption::Some(x),
      MOption::None => MOption::None,
    }
  }

  /// &mut Option<T> -> Option<&mut T>
  #[inline]
  pub fn as_mut(&mut self) -> MOption<&mut T> {
    match *self {
      MOption::Some(ref mut x) => MOption::Some(x),
      MOption::None => MOption::None,
    }
  }

  #[inline]
  pub fn expect(self, msg: &str) -> T {
    match self {
      MOption::Some(val) => val,
      MOption::None => expect_failed(msg),
    }
  }

  #[inline]
  pub fn unwrap(self) -> T {
    match self {
      MOption::Some(val) => val,
      MOption::None => panic!("called `MOption::unwrap()` on a `None` value"),
    }
  }

  #[inline]
  pub fn unwrap_or(self, default: T) -> T {
    match self {
      MOption::Some(x) => x,
      MOption::None => default,
    }
  }

  #[inline]
  pub fn unwrap_or_else<F: FnOnce() -> T>(self, f: F) -> T {
    match self {
      MOption::Some(x) => x,
      MOption::None => f(),
    }
  }

  /// Option<T> -> Option<U>
  #[inline]
  pub fn map<U, F: FnOnce(T) -> U>(self, f: F) -> MOption<U> {
    match self {
      MOption::Some(x) => MOption::Some(f(x)),
      MOption::None => MOption::None,
    }
  }

  #[inline]
  pub fn map_or<U, F: FnOnce(T) -> U>(self, default: U, f: F) -> U {
    match self {
      MOption::Some(x) => f(x),
      MOption::None => default,
    }
  }

  #[inline]
  pub fn map_or_else<U, D: FnOnce() -> U, F: FnOnce(T) -> U>(self, default: D, f: F) -> U {
    match self {
      MOption::Some(x) => f(x),
      MOption::None => default(),
    }
  }

  /// Option<T> -> Result<T, E>
  #[inline]
  pub fn ok_or<E>(self, err: E) -> Result<T, E> {
    match self {
      MOption::Some(v) => Ok(v),
      MOption::None => Err(err),
    }
  }

  /// Option<T> -> Result<T, E>
  #[inline]
  pub fn ok_or_else<E, F: FnOnce() -> E>(self, err: F) -> Result<T, E> {
    match self {
      MOption::Some(v) => Ok(v),
      MOption::None => Err(err()),
    }
  }

  #[inline]
  pub fn and<U>(self, optb: MOption<U>) -> MOption<U> {
    match self {
      MOption::Some(_) => optb,
      MOption::None => MOption::None,
    }
  }

  /// Option<T> -> Option<U>
  #[inline]
  pub fn and_then<U, F: FnOnce(T) -> MOption<U>>(self, f: F) -> MOption<U> {
    match self {
      MOption::Some(x) => f(x),
      MOption::None => MOption::None,
    }
  }

  #[inline]
  pub fn filter<P: FnOnce(&T) -> bool>(self, predicate: P) -> Self {
    if let MOption::Some(x) = self {
      if predicate(&x) {
        return MOption::Some(x);
      }
    }
    MOption::None
  }

  #[inline]
  pub fn or(self, optb: MOption<T>) -> MOption<T> {
    match self {
      MOption::Some(_) => self,
      MOption::None => optb,
    }
  }

  #[inline]
  pub fn or_else<F: FnOnce() -> MOption<T>>(self, f: F) -> MOption<T> {
    match self {
      MOption::Some(_) => self,
      MOption::None => f(),
    }
  }

  #[inline]
  pub fn xor(self, optb: MOption<T>) -> MOption<T> {
    match (self, optb) {
      (MOption::Some(a), MOption::None) => MOption::Some(a),
      (MOption::None, MOption::Some(b)) => MOption::Some(b),
      _ => MOption::None,
    }
  }

  #[inline]
  pub fn take(&mut self) -> MOption<T> {
    mem::take(self)
  }

  #[inline]
  pub fn replace(&mut self, value: T) -> MOption<T> {
    mem::replace(self, MOption::Some(value))
  }

  #[inline]
  pub fn zip<U>(self, other: MOption<U>) -> MOption<(T, U)> {
    match (self, other) {
      (MOption::Some(a), MOption::Some(b)) => MOption::Some((a, b)),
      _ => MOption::None,
    }
  }

  //#[inline]
  //pub fn zip_with<U, F, R>(self, other: MOption<U>, f: F) -> MOption<R>
  //where
  //  F: FnOnce(T, U) -> R,
  //{
  //  MOption::Some(f(self?, other?))
  //}
}

fn expect_failed(msg: &str) -> ! {
  panic!("{}", msg)
}

fn expect_none_failed(msg: &str, value: &dyn fmt::Debug) -> ! {
  panic!("{}: {:?}", msg, value)
}

impl<T: Clone> Clone for MOption<T> {
  #[inline]
  fn clone(&self) -> Self {
    match self {
      MOption::Some(x) => MOption::Some(x.clone()),
      MOption::None => MOption::None,
    }
  }

  #[inline]
  fn clone_from(&mut self, source: &Self) {
    match (self, source) {
      (MOption::Some(to), MOption::Some(from)) => to.clone_from(from),
      (to, from) => *to = from.clone(),
    }
  }
}

impl<T> Default for MOption<T> {
  #[inline]
  fn default() -> MOption<T> {
    MOption::None
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_moption_is_some() {
    let x: MOption<u32> = MOption::Some(2);
    assert_eq!(x.is_some(), true);

    let x: MOption<u32> = MOption::None;
    assert_eq!(x.is_some(), false);
  }

  #[test]
  fn test_moption_is_none() {
    let x: MOption<u32> = MOption::Some(2);
    assert_eq!(x.is_none(), false);

    let x: MOption<u32> = MOption::None;
    assert_eq!(x.is_none(), true);
  }

  #[test]
  fn test_moption_contains() {
    let x: MOption<u32> = MOption::Some(2);
    assert_eq!(x.contains(&2), true);

    let x: MOption<u32> = MOption::Some(3);
    assert_eq!(x.contains(&2), false);

    let x: MOption<u32> = MOption::None;
    assert_eq!(x.contains(&2), false);
  }

  #[test]
  fn test_moption_as_mut() {
    let mut x = MOption::Some(2);
    match x.as_mut() {
      MOption::Some(v) => *v = 42,
      MOption::None => {}
    }
    assert_eq!(x, MOption::Some(42));
  }

  #[test]
  fn test_moption_unwrap_or() {
    assert_eq!(Some("car").unwrap_or("bike"), "car");
    assert_eq!(None.unwrap_or("bike"), "bike");
  }

  #[test]
  fn test_moption_unwrap_or_else() {
    let k = 10;
    assert_eq!(Some(4).unwrap_or_else(|| 2 * k), 4);
    assert_eq!(None.unwrap_or_else(|| 2 * k), 20);
  }

  #[test]
  fn test_moption_map() {
    let maybe_some_string = Some(String::from("Hello, World!"));
    let maybe_some_len = maybe_some_string.map(|s| s.len());
    assert_eq!(maybe_some_len, Some(13));
  }

  #[test]
  fn test_moption_map_or() {
    let x = MOption::Some("foo");
    assert_eq!(x.map_or(42, |v| v.len()), 3);

    let x: MOption<&str> = MOption::None;
    assert_eq!(x.map_or(42, |v| v.len()), 42);
  }

  #[test]
  fn test_moption_map_or_else() {
    let k = 21;

    let x = MOption::Some("foo");
    assert_eq!(x.map_or_else(|| 2 * k, |v| v.len()), 3);

    let x: MOption<&str> = MOption::None;
    assert_eq!(x.map_or_else(|| 2 * k, |v| v.len()), 42);
  }

  #[test]
  fn test_moption_ok_or() {
    let x = MOption::Some("foo");
    assert_eq!(x.ok_or(0), Ok("foo"));

    let x: MOption<&str> = MOption::None;
    assert_eq!(x.ok_or(0), Err(0));
  }

  #[test]
  fn test_moption_ok_or_else() {
    let x = MOption::Some("foo");
    assert_eq!(x.ok_or_else(|| 0), Ok("foo"));

    let x: MOption<&str> = MOption::None;
    assert_eq!(x.ok_or_else(|| 0), Err(0));
  }

  #[test]
  fn test_moption_and() {
    let x = MOption::Some(2);
    let y: MOption<&str> = MOption::None;
    assert_eq!(x.and(y), MOption::None);

    let x: MOption<u32> = MOption::None;
    let y = MOption::Some("foo");
    assert_eq!(x.and(y), MOption::None);

    let x = MOption::Some(2);
    let y = MOption::Some("foo");
    assert_eq!(x.and(y), MOption::Some("foo"));
  }

  #[test]
  fn test_moption_and_then() {
    fn sq(x: u32) -> MOption<u32> {
      MOption::Some(x * x)
    }

    fn nope(_: u32) -> MOption<u32> {
      MOption::None
    }

    assert_eq!(
      MOption::Some(2).and_then(sq).and_then(sq),
      MOption::Some(16)
    );

    assert_eq!(MOption::Some(2).and_then(sq).and_then(nope), MOption::None);
  }

  #[test]
  fn test_moption_filter() {
    fn is_even(n: &i32) -> bool {
      n % 2 == 0
    }

    assert_eq!(MOption::None.filter(is_even), MOption::None);
    assert_eq!(MOption::Some(3).filter(is_even), MOption::None);
    assert_eq!(MOption::Some(4).filter(is_even), MOption::Some(4));
  }

  #[test]
  fn test_moption_or() {
    let x = MOption::Some(2);
    let y = MOption::None;
    assert_eq!(x.or(y), MOption::Some(2));

    let x = MOption::None;
    let y = MOption::Some(100);
    assert_eq!(x.or(y), MOption::Some(100));

    let x = MOption::Some(2);
    let y = MOption::Some(100);
    assert_eq!(x.or(y), MOption::Some(2));
  }

  #[test]
  fn test_moption_or_else() {
    fn nobody() -> MOption<&'static str> {
      MOption::None
    }
    fn vikings() -> MOption<&'static str> {
      MOption::Some("vikings")
    }

    assert_eq!(
      MOption::Some("barbarians").or_else(vikings),
      MOption::Some("barbarians")
    );

    assert_eq!(MOption::None.or_else(vikings), MOption::Some("vikings"));
    assert_eq!(MOption::None.or_else(nobody), MOption::None);
  }

  #[test]
  fn test_moption_xor() {
    let x = MOption::Some(2);
    let y: MOption<u32> = MOption::None;
    assert_eq!(x.xor(y), MOption::Some(2));

    let x: MOption<u32> = MOption::None;
    let y = MOption::Some(2);
    assert_eq!(x.xor(y), MOption::Some(2));

    let x = MOption::Some(2);
    let y = MOption::Some(2);
    assert_eq!(x.xor(y), MOption::None);

    let x: MOption<u32> = MOption::None;
    let y: MOption<u32> = MOption::None;
    assert_eq!(x.xor(y), MOption::None);
  }

  #[test]
  fn test_moption_take() {
    let mut x = MOption::Some(2);
    let y = x.take();
    assert_eq!(x, MOption::None);
    assert_eq!(y, MOption::Some(2));

    let mut x: MOption<u32> = MOption::None;
    let y = x.take();
    assert_eq!(x, MOption::None);
    assert_eq!(y, MOption::None);
  }

  #[test]
  fn test_moption_replace() {
    let mut x = MOption::Some(2);
    let old = x.replace(5);
    assert_eq!(x, MOption::Some(5));
    assert_eq!(old, MOption::Some(2));

    let mut x = MOption::None;
    let old = x.replace(3);
    assert_eq!(x, MOption::Some(3));
    assert_eq!(old, MOption::None);
  }

  #[test]
  fn test_moption_zip() {
    let x = MOption::Some(1);
    let y = MOption::Some("hi");
    let z = MOption::None::<u8>;

    assert_eq!(x.zip(y), MOption::Some((1, "hi")));
    assert_eq!(x.zip(z), MOption::None);
  }
}
