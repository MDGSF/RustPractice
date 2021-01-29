// Lifetime names for struct fields always need to be declared after the impl keyword and then used after the struct’s name, because those lifetimes are part of the struct’s type.

#[derive(Debug)]
struct ImportantExcerpt<'a> {
  part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
  // impl<'a> ImportantExcerpt<'a>
  // fn level(&self) -> i32;           ok
  // fn level(&'a self) -> i32;        ok
  // fn level<'a>(&'a self) -> i32;    failed
  // fn level(&'b self) -> i32;        failed
  // fn level<'b>(&'b self) -> i32;    ok
  //
  // impl <'a, 'b> ImportantExcerpt<'a>
  // fn level(&'b self) -> i32;        ok
  fn level(&self) -> i32 {
    3
  }

  // impl<'a> ImportantExcerpt<'a>
  // fn nrp(&self, announcement: &str) -> &str;                  ok
  // fn nrp<'b>(&'a self, announcement: &'b str) -> &'a str;     ok
  // fn nrp(&'a self, announcement: &'a str) -> &'a str;         ok
  // fn nrp<'b>(&'a self, announcement: &'b str) -> &'b str;     failed
  // fn nrp<'b>(&'b self, announcement: &'b str) -> &'b str;     ok
  // fn nrp<'b, 'c>(&'b self, announcement: &'c str) -> &'b str; ok
  //
  // impl <'a, 'b> ImportantExcerpt<'a>
  // fn nrp(&'a self, announcement: &'b str) -> &'a str;         ok
  fn nrp(&self, announcement: &str) -> &str {
    println!("Attention please: {}", announcement);
    self.part
  }
}

fn main() {
  let noval = String::from("Call me Ishmael. Some years ago...");
  let first_sentence = noval.split('.').next().expect("Could not find a '.'");
  let i = ImportantExcerpt {
    part: first_sentence,
  };
  println!("i = {:?}", i);
}
