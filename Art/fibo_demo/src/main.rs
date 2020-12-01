fn fibo_1(n: i32) -> i32 {
  if n <= 1 {
    return 1;
  }
  fibo_1(n - 1) + fibo_1(n - 2)
}

fn fibo_2(n: i32) -> i32 {
  if n <= 1 {
    return 1;
  }
  let mut a = 1;
  let mut b = 1;
  for _ in 1..n {
    let t = b;
    b += a;
    a = t;
  }
  b
}

fn fibo_3(n: i32) -> i32 {
  fibo_3_inner(n, 1, 1)
}

fn fibo_3_inner(n: i32, a: i32, b: i32) -> i32 {
  if n <= 1 {
    return b;
  }
  fibo_3_inner(n - 1, b, a + b)
}

fn main() {
  for i in 0..8 {
    println!("{}", fibo_1(i))
  }
  println!();

  for i in 0..8 {
    println!("{}", fibo_2(i))
  }
  println!();

  for i in 0..8 {
    println!("{}", fibo_3(i))
  }
  println!();
}
