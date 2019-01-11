fn main() {
    println!("Hello, world!");
    let vector = vec!["fli", "faa", "fbb"];
    println!("{}", longest_common_prefix(vector))
}

pub fn longest_common_prefix(strs: Vec<String>) -> String {
    let mut prefix = "".to_string();

    let mut min_len = 0;

    for x in strs.iter() {
        if x.len() < min_len {
            min_len = x.len();
        }
    }

    let mut i = 0;
    while i < min_len {
        let mut j = 0;
        while j < strs.len() - 1 {
            let a = strs[j].as_bytes();
            let b = strs[j + 1].as_bytes();
            if a[i] != b[i] {
                return prefix.to_string();
            }
            j = j + 1;
        }
        let a = strs[0].as_bytes();
        prefix = prefix + &a[i].to_string();

        i = i + 1;
    }

    prefix.to_string()
}
