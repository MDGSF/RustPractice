extern crate clap;

use clap::App;

fn main() {
  App::new("MyApp")
    .version("1.0")
    .author("HuangJian <1342042894@qq.com>")
    .about("Does awesome things")
    .get_matches();
}
