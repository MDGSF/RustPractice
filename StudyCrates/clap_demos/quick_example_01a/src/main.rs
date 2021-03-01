extern crate clap;

use clap::{App, SubCommand};

fn main() {
  let matches = App::new("MyApp")
    .version("1.0")
    .author("HuangJian <1342042894@qq.com>")
    .about("Does awesome things")
    .args_from_usage(
      "-c, --config=[FILE] 'Sets a custom config file'
       <output> 'Sets an optional output file'
       -d... 'Turn debugging information on'",
    )
    .subcommand(
      SubCommand::with_name("test")
        .about("does testing things")
        .arg_from_usage("-l, --list 'lists test values'"),
    )
    .get_matches();

  if let Some(o) = matches.value_of("output") {
    println!("Value for output: {}", o)
  }

  if let Some(o) = matches.value_of("config") {
    println!("Value for config: {}", o)
  }

  match matches.occurrences_of("d") {
   0 => println!("Debug mode is off"), 
   1 => println!("Debug mode is kind of on"), 
   2 => println!("Debug mode is on"), 
   3 | _ => println!("Don't be crazy"), 
  }

  if let Some(matches) = matches.subcommand_matches("test") {
    if matches.is_present("list") {
      println!("Printing testing lists...")
    } else {
      println!("Not printing testing lists...")
    }
  }
}
