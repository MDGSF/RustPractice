fn main() {
  test1();
  test2();
}

fn test1() {
  println!("He said, \"You can find the file at c:\\files\\my_documents\\file.txt.\" Then I found the file."); // We used \ five times here
  println!(
    r#"He said, "You can find the file at c:\files\my_documents\file.txt." Then I found the file."#
  )
}

fn test2() {
  let my_string = "'Ice to see you,' he said."; // single quotes
  let quote_string = r#""Ice to see you," he said."#; // double quotes
  let hashtag_string = r##"The hashtag #IceToSeeYou had become very popular."##; // Has one # so we need at least ##
  let many_hashtags = r####""You don't have to type ### to use a hashtag. You can just use #.""####; // Has three ### so we need at least ####

  println!(
    "{}\n{}\n{}\n{}\n",
    my_string, quote_string, hashtag_string, many_hashtags
  );
}
