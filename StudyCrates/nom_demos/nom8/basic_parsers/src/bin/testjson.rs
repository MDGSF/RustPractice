use basic_parsers::json::root;
use nom::error::ErrorKind;
use nom_language::error::{VerboseError, convert_error};

fn main() {
    let data = "  { \"a\"\t: 42,
  \"b\": [ \"x\", \"y\", 12 ] ,
  \"c\": { 1\"hello\" : \"world\"
  }
  } ";

    println!(
        "will try to parse invalid JSON data:\n\n**********\n{}\n**********\n",
        data
    );

    println!(
        "basic errors - `root::<(&str, ErrorKind)>(data)`:\n{:#?}\n",
        root::<(&str, ErrorKind)>(data)
    );

    println!(
        "parsed verbose: {:#?}\n\n",
        root::<VerboseError<&str>>(data)
    );

    match root::<VerboseError<&str>>(data) {
        Err(nom::Err::Error(e)) | Err(nom::Err::Failure(e)) => {
            println!(
                "verbose errors - `root::<VerboseError>(data)`:\n{}",
                convert_error(data, e)
            );
        }
        _ => {}
    }
}
