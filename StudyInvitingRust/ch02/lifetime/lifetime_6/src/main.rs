// https://dtolnay.github.io/rust-quiz/11
// Generic parameters can be either early bound or late bound.
// Currently type parameters are always early bound,
// but lifetime parameters can be either early or late bound.
//
// let m1: impl for<'r> Fn(&'r ()) = m;
// You can think of this as meaning:
// "There is a lifetime but we don't need to know what it is just yet".
//
// Lifetimes on data types are always early bound except when the developer
// has explicitly used the HRTB for syntax. On functions, lifetimes are
// late bound by default but can be early bound if:

fn f<'a>() {}
fn g<'a: 'a>() {}

fn main() {
  let pf = f::<'static> as fn(); // late bound
  let pg = g::<'static> as fn(); // early bound

  println!("{}", pf == pg);
}
