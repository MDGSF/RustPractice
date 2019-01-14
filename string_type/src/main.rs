use std::mem;
use std::str;

fn main() {
    data_type();

    string_type();
}

fn string_type() {
    println!("\nhello string: {:?}", "hello world");
    println!("hello byte string: {:?}", b"hello world");

    let array_of_u8 = [104, 101, 108, 108, 111, 32, 119, 111, 114, 108, 100];
    println!("array_of_u8 = {:?}", array_of_u8);

    // [u8] to String
    let string_utf8_lossy = String::from_utf8_lossy(&array_of_u8);
    println!("string_utf8_lossy = {}", string_utf8_lossy);

    // [u8] to String
    let mut vec_of_u8 = vec![];
    vec_of_u8.extend_from_slice(&array_of_u8);
    let string_utf8_result = String::from_utf8(vec_of_u8).unwrap();
    println!("string_utf8_result = {}", string_utf8_result);

    // [u8] to str
    let str_utf8_result = str::from_utf8(&array_of_u8).unwrap();
    println!("str_utf8_result = {}", str_utf8_result);

    // [u8] to Vec<char>
    let vec_of_chars: Vec<char> = array_of_u8.iter().map(|byte| *byte as char).collect();
    println!("vec_of_chars = {:?}", vec_of_chars);

    // Vec<char> to Vec<u8>
    let vec_of_u8s: Vec<u8> = vec_of_chars.iter().map(|c| *c as u8).collect();
    println!("vec_of_u8s = {:?}", vec_of_u8s);

    // Vec<char> to String
    let mut string_of_collected_chars: String = vec_of_chars.iter().collect();
    println!("string_of_collected_chars = {}", string_of_collected_chars);

    string_of_collected_chars.push('!');
    string_of_collected_chars.push_str("!!");
    println!("string_of_collected_chars = {}", string_of_collected_chars);

    // String to str
    let str_slice = &string_of_collected_chars[..];
    println!("str_slice = {}", &str_slice);

    // String to [u8]
    let array_of_u8_from_string = string_of_collected_chars.as_bytes();
    println!("array_of_u8_from_string = {:?}", array_of_u8_from_string);

    // String to Vec<char>
    let vec_of_chars_from_string: Vec<char> = string_of_collected_chars.chars().collect();
    println!("vec_of_chars_from_string = {:?}", vec_of_chars_from_string);

    // merge string
    let concat_strings = vec!["abc".to_string(), "def".to_string()].concat();
    println!("concat_strings = {}", concat_strings);

    let joined_strings = vec!["abc".to_string(), "def".to_string()].join("---");
    println!("joined_strings = {}", joined_strings);

    // str to String
    let a = "hello world";
    let b = "ok!";
    let c = a.to_string();
    let d = String::from(b);
    let e = a.to_owned();
    println!("a = {}", a);
    println!("b = {}", b);
    println!("c = {}", c);
    println!("d = {}", d);
    println!("e = {}", e);

    // String + &str ==> String
    let f = c + " --- " + b;
    println!("f = {}", f);

    // f32 + String ==> String
    let num = 2019_u32;
    let new_year = String::from(", happy new year");
    let num_str = format!("{0}{1}", num, new_year);
    println!("num_str = {}", num_str);

    // char to String
    let single_char: char = 'a';
    let string_from_char: String = single_char.to_string();
    println!("single_char = {}", single_char);
    println!("string_from_char = {}", string_from_char);

    // i32 to String
    let single_i32: i32 = 32;
    let string_from_i32: String = single_i32.to_string();
    println!("string_from_i32 = {}", string_from_i32);

    // String to i32
    // let i32_from_string: i32 = string_from_i32.parse().unwrap();
    // let i32_from_string = string_from_i32.parse::<i32>().unwrap();
    let i32_from_string: i32 = string_from_i32.parse::<i32>().unwrap();
    println!("i32_from_string = {}", i32_from_string);

    // vec<&str> to vec<String>
    let v = vec!["huangjian", "minieye"];
    let vec_string = vec_str_to_vec_string(v);
    println!("vec_string = {:?}", vec_string);
}

fn vec_str_to_vec_string(strs: Vec<&str>) -> Vec<String> {
    println!("strs = {:?}", strs);

    let mut result = Vec::new();
    for i in &strs {
        result.push((*i).to_string());
    }
    result
}

fn data_type() {
    println!("Show Data Type");

    let single_i8: i8 = 8;
    let single_i16: i16 = 16;
    let single_i32: i32 = 32;
    let single_i64: i64 = 64;
    let single_i128: i128 = 128;

    println!("single_i8 = {}", single_i8);
    println!("single_i16 = {}", single_i16);
    println!("single_i32 = {}", single_i32);
    println!("single_i64 = {}", single_i64);
    println!("single_i128 = {}", single_i128);

    let single_u8: u8 = 8;
    let single_u16: u16 = 16;
    let single_u32: u32 = 32;
    let single_u64: u64 = 64;
    let single_u128: u128 = 128;

    println!("single_u8 = {}", single_u8);
    println!("single_u16 = {}", single_u16);
    println!("single_u32 = {}", single_u32);
    println!("single_u64 = {}", single_u64);
    println!("single_u128 = {}", single_u128);

    println!("Decimal = {}", 98_222);
    println!("Hex = {}", 0xff);
    println!("Octal = {}", 0o77);
    println!("Binary = {}", 0b1111_0000);
    println!("Byte(u8) = {}", b'A');

    let single_f32: f32 = 32.0;
    let single_f64: f64 = 64.0;
    println!("single_f32 = {}", single_f32);
    println!("single_f64 = {}", single_f64);

    // i32 to i64
    let i64_from_i32 = single_i32 as i64;
    println!("i64_from_i32 = {}", i64_from_i32);

    // i32 to u32
    let u32_from_i32 = single_i32 as u32;
    println!("u32_from_i32 = {}", u32_from_i32);

    // i32 to f64
    let f64_from_i32 = single_i32 as f64;
    println!("f64_from_i32 = {}", f64_from_i32);

    // transmute is dangerous.
    unsafe {
        let a = [127u8, 0u8, 0u8, 1u8];
        let b = mem::transmute::<[u8; 4], u32>(a);
        println!("b_transmute_from_a = {}", b);
    }
}
