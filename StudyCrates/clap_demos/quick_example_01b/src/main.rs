extern crate clap;

use clap::{App, Arg, SubCommand};

fn main() {
  let matches = App::new("MyApp")
    .version("1.0")
    .author("HuangJian <1342042894@qq.com>")
    .about("Does awesome things")
    .arg(
      Arg::with_name("config")
        .short("c")
        .long("config")
        .value_name("FILE")
        .help("Sets a custom config file")
        .takes_value(true),
    )
    .arg(
      Arg::with_name("output")
        .help("Sets an optional output file")
        .index(1),
    )
    .arg(
      Arg::with_name("debug")
        .short("d")
        .multiple(true)
        .help("Turn debugging information on"),
    )
    .subcommand(
      SubCommand::with_name("test")
        .about("does testing things")
        .arg(Arg::with_name("list").short("l").help("lists test values")),
    )
    .get_matches();

  if let Some(o) = matches.value_of("output") {
    println!("Value for output: {}", o)
  }

  if let Some(o) = matches.value_of("config") {
    println!("Value for config: {}", o)
  }

  match matches.occurrences_of("debug") {
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
