#[macro_use]
extern crate clap;

fn main() {
  let matches = clap_app!(myapp =>
    (version: "1.0")
    (author: "HuangJian <1342042894@qq.com>")
    (about: "Does awesome things")
    (@arg CONFIG: -c --config +takes_value "Sets a custom config file")
    (@arg INPUT: +required "Sets the input file to use")
    (@arg debug: -d ... "Turn debugging information on")
    (@subcommand test =>
      (about: "does testing things")
      (version: "1.0")
      (author: "Someone E. <someone_else@other.com>")
      (@arg verbose: -v --verbose "Print test information verbosely"))
  )
  .get_matches();

  println!("Using input file: {}", matches.value_of("INPUT").unwrap());

  let config = matches.value_of("CONFIG").unwrap_or("default.conf");
  println!("Value for config: {}", config);

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
