fn main() {
  let mut my_string = String::from(".daer ot drah tib elttil a si gnirts sihT");
  loop {
    let pop_result = my_string.pop();
    match pop_result {
      Some(character) => print!("{}", character),
      None => break,
    }
  }
}
