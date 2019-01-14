fn sqr(i: i32) -> i32 {
    i * i
}

fn abs(i: i32) -> i32 {
    if i >= 0 {
        i
    } else {
        -i
    }
}

enum NumberOrNothing {
    Number(i32),
    Nothing,
}

impl NumberOrNothing {
    fn print(self) {
        match self {
            Nothing => println!("The number is: <nothing>"),
            Number(n) => println!("The number is: {}", n),
        }
    }
}

use self::NumberOrNothing::{Nothing, Number};

fn number_or_default(n: NumberOrNothing, default: i32) -> i32 {
    match n {
        Nothing => default,
        Number(n) => n,
    }
}

fn compute_stuff(x: i32) -> i32 {
    let y = {
        let z = x * x;
        z + 14
    };
    y * y
}

fn vec_min(v: &Vec<i32>) -> NumberOrNothing {
    fn min_i32(a: i32, b: i32) -> i32 {
        if a < b {
            a
        } else {
            b
        }
    }

    let mut min = Nothing;
    for e in v {
        let curE = *e;
        min = Number(match min {
            Nothing => curE,
            Number(n) => min_i32(n, curE),
        });
    }
    min
}

fn read_vec() -> Vec<i32> {
    vec![18, 5, 7, 2, 9, 27]
}

fn vec_sum(v: &Vec<i32>) -> NumberOrNothing {
    let mut sum = Nothing;
    for e in v {
        let curE = *e;
        sum = Number(match sum {
            Nothing => curE,
            Number(n) => n + curE,
        });
    }
    sum
}

fn vec_print(v: &Vec<i32>) {
    for e in v {
        print!("{} ", e)
    }
    println!()
}

fn main() {
    let vec = read_vec();
    vec_print(&vec);

    let min = vec_min(&vec);
    let sum = vec_sum(&vec);
    min.print();
    sum.print();
}
