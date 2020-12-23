fn compute(input: &u32, output: &mut u32) {
    if *input > 10 {
        *output = 1;
    }
    if *input > 5 {
        *output *= 2;
    }
}

fn main() {
    let i = 20;
    let mut o = 5;
    compute(&i, &mut o);
    println!("i = {}", i);
    println!("o = {}", o);
}
