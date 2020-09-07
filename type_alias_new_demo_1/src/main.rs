type File1 = String;

struct File2(String);

fn main() {
  let my_file1 = File1::from("I am file contents");
  let my_file2 = File2(String::from("I am file contents"));
  let my_string = String::from("I am file contents");
  println!("{}", my_file1 == my_string);
  // can't compare, because they are two different types.
  // println!("{}", my_file2 == my_string);
  println!("{}", my_file2.0 == my_string);
}
