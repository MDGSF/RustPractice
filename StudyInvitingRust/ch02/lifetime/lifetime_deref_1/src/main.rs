/*
pub trait Deref {
  type Target: ?Sized;
  pub fn deref(&self) -> &Self::Target;
}
pub trait DerefMut: Deref {
  pub fn deref_mut(&mut self) -> &mut Self::Target;
}

pub trait Clone {
  pub fn clone(&self) -> Self;
  pub fn clone_from(&mut self, source: &Self) { ... }
}
pub trait Copy: Clone { }

&'b mut &'a mut T
 -->(ok) &'b mut T
 -->(no) &'a mut T

pub fn deref_mut<'b>(&'b mut self) -> &'b mut Self::Target;
pub fn deref_mut<'b>(&'b mut &'a mut T) -> &'b mut T;

&'b mut &'a T
 -->(ok) &'b &'a T
 -->(ok) &'b T
 -->(ok) &'a T

pub fn clone<'b>(&'b self) -> Self;
pub fn clone<'b>(&'b &'a T) -> &'a T;
*/

use std::fmt::Debug;

fn foo<'a: 'b, 'b, T>(x: &'b mut &'a mut T)
where
  T: Debug,
{
  let a: &'b mut T = x;
  println!("foo a = {:?}", a);

  //let b: &'a mut T = x;
  //println!("b = {:?}", b);
}

fn bar<'a: 'b, 'b, T>(x: &'b mut &'a T)
where
  T: Debug,
{
  let a: &'b &'a T = x;
  println!("bar a = {:?}", a);

  let b: &'b T = x;
  println!("bar b = {:?}", b);

  let c: &'a T = x;
  println!("bar c = {:?}", c);

  //let d: &'a &'b T = x;
  //println!("bar d = {:?}", d);
}

fn main() {
  {
    let mut a = 10;
    foo(&mut &mut a);
  }

  {
    let mut a = 10;
    bar(&mut &a);
  }
}
