use self::NumberOrNothing::{Nothing, Number};

enum NumberOrNothing {
    Number(i32),
    Nothing,
}

fn vec_min(vec: Vec<i32>) -> NumberOrNothing {
    let mut min = NumberOrNothing::Nothing;

    for el in vec {
        match min {
            NumberOrNothing::Nothing => {
                min = NumberOrNothing::Number(el);
            }
            NumberOrNothing::Number(n) => {
                let new_min = min_i32(n, el);
                min = NumberOrNothing::Number(new_min);
            }
        }
    }

    return min;
}

fn min_i32(a: i32, b: i32) -> i32 {
    if a < b {
        return a;
    } else {
        return b;
    }
}

fn read_vec() -> Vec<i32> {
    vec![18, 5, 7, 1, 9, 27]
}

fn print_number_or_nothing(n: NumberOrNothing) {
    match n {
        Nothing => println!("The number is: <nothing>"),
        Number(n) => println!("The number is: {}", n),
    };
}

fn main() {
    let vec = read_vec();
    let min = vec_min(vec);
    print_number_or_nothing(min);
}
