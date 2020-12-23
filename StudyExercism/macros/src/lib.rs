#[macro_export(local_inner_macros)]
macro_rules! hashmap {
  ($($key:expr => $val:expr,)+) => { hashmap!($($key => $val),+) };
  ($($key:expr => $val:expr),*) => {{
    let mut hm = ::std::collections::HashMap::new();
    $(
      hm.insert($key, $val);
    )*
    hm
  }};
}
