macro_rules! m {
  ($($s:stmt)*) => {
    $(
      { stringify!($s); 1}
    )<<*
  };
}

fn main() {
  println!(
    "{}{}{}",
    m! { return || true },
    m! { (return) || true },
    m! { {return} || true },
  );
}
