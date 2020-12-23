fn main() {
    println!("() = {}", is_valid(String::from("()")));
    println!("(() = {}", is_valid(String::from("(()")));
    println!("{{}} = {}", is_valid(String::from("{{}}")));
}

fn is_valid(s: String) -> bool {
    let mut v = Vec::new();

    let bytes: Vec<char> = s.chars().collect();
    for c in bytes.iter() {
        if *c == '(' || *c == '[' || *c == '{' {
            v.push(c);
        } else {
            if v.len() == 0 {
                return false;
            }

            let t = v.pop();
            if !((*c == ')' && t == Some(&'('))
                || (*c == ']' && t == Some(&'['))
                || (*c == '}' && t == Some(&'{')))
            {
                return false;
            }
        }
    }

    if v.len() != 0 {
        return false;
    }

    true
}
