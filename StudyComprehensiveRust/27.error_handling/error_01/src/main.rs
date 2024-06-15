use std::fs;
use std::io::Read;

fn main() {
    let file = fs::File::open("diary.txt");
    match file {
        Ok(mut file) => {
            let mut contents = String::new();
            file.read_to_string(&mut contents);
            println!("Dear diary: {contents}");
        }
        Err(err) => {
            println!("The diary could not be opened: {err}");
        }
    }
}
