fn main() {
    let arr: [i32; 3] = [1, 2, 3];
    let mut mut_arr = [1, 2, 3];

    println!("arr = {:?}", arr);
    println!("mut_arr = {:?}", mut_arr);

    mut_arr[0] = 3;
    println!("arr = {:?}", arr);
    println!("mut_arr = {:?}", mut_arr);

    let init_arr = [0; 10];
    println!("init_arr = {:?}", init_arr);
    println!("init_arr.len() = {}", init_arr.len());
}
