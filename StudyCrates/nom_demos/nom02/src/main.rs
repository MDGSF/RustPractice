use nom02::mount_parsers;
use nom02::BoxError;
use std::io::BufRead;

fn main() -> Result<(), BoxError> {
    let file = std::fs::File::open("/proc/mounts")?;
    let buf_reader = std::io::BufReader::new(file);
    for line in buf_reader.lines() {
        match mount_parsers::parse_line(&line?[..]) {
            Ok((_, m)) => {
                println!("{:?}", m);
            }
            Err(_) => {}
        }
    }
    Ok(())
}
