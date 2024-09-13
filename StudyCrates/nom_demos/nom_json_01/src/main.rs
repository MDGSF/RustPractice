use nom_json_01::json_parser::parse_json;

fn main() {
    println!("Hello, world!");
    let file_path = "test.json";
    let data = std::fs::read_to_string(file_path).unwrap();
    let result = parse_json(&data).unwrap();
    println!("{:?}", result);
}
