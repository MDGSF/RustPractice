// 查找字符串中的 ? ，用字母替换，该字母不能和左右两边的字母一样。
fn main() {
  let s = "abc?d";
  let mut chars = s.chars().collect::<Vec<char>>();

  for i in 0..chars.len() {
    let mut words = ('a'..='z').into_iter();

    if chars[i] == '?' {
      let left = if i == 0 { None } else { Some(chars[i - 1]) };
      let right = if i == s.len() - 1 {
        None
      } else {
        Some(chars[i + 1])
      };

      chars[i] = words
        .find(|&w| Some(w) != left && Some(w) != right)
        .unwrap();
    }
  }

  let s = chars.into_iter().collect::<String>();
  println!("{:?}", s);
}
