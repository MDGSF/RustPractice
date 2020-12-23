fn main() {
    let outer_val = 1;
    let outer_sp = "hello".to_string();
    {
        let inner_val = 2;
        outer_val;
        outer_sp;
    }
    println!("{:?}", outer_val);
    //println!("{:?}", outer_sp);
}
