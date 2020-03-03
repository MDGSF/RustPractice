// https://wiki.jikexueyuan.com/project/rust-primer/match/pattern.html

struct Point {
  x: i64,
  y: i64,
}

fn main() {
  let tup = (0, 1);
  let (x, y) = tup;
  println!("x = {}, y = {}", x, y);

  let point = Point { x: 9, y: 10 };
  match point {
    Point { x, y } => println!("({}, {})", x, y),
  }

  match point {
    Point { x: x1, y: y1 } => println!("({}, {})", x1, y1),
  }

  match point {
    Point { y, .. } => println!("y = {}", y),
  }

  let tuple: (u32, String) = (5, String::from("five"));
  // tuple.1 的所有权会转移到 s 上
  let (x, s) = tuple;
  // println!("tuple = {:?}", tuple);

  let tuple = (5, String::from("five"));
  let (x, _) = tuple;
  println!("tuple = {:?}", tuple);

  //当被模式匹配命中的时候，未实现Copy的类型会被默认的move掉，
  //因此，原owner就不再持有其所有权。但是有些时候，我们只想要从中拿到一个
  //变量的（可变）引用，而不想将其move出作用域，怎么做呢？答：用ref或者ref mut。

  let mut x = 5;
  match x {
    ref mut mr => println!("ref mut mr: {}", mr),
  }
  println!("x = {}", x);

  let s = String::from("hello");
  match s {
    ref ns => println!("mut ns: {}", ns),
  }
  println!("s = {}", s);

  // 在模式匹配的过程内部，我们可以用@来绑定一个变量名
  let x = 12;
  match x {
    e @ 1...5 | e @ 10...15 => println!("get: {}", e),
    _ => (),
  }

  #[derive(Debug)]
  struct Person {
    name: Option<String>,
  }
  let name = "Steve".to_string();
  let x: Option<Person> = Some(Person { name: Some(name) });
  match x {
    Some(Person {
      name: ref a @ Some(_),
      ..
    }) => println!("{:?}", a),
    _ => (),
  }

  // 一个后置的if表达式可以被放在match的模式之后，被称为match guards。
  let x = 4;
  let y = false;
  match x {
    4 | 5 if y => println!("yes"),
    _ => println!("no"),
  }
}
