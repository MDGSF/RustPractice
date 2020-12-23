fn bubble_sort(a: &mut Vec<i32>) {
    let mut n = a.len();
    while n > 0 {
        let (mut i, mut max_ptr) = (1, 0);

        while i < n {
            if a[i - 1] > a[i] {
                a.swap(i - 1, i);
                max_ptr = i;
            }
            i += 1;
        }

        n = max_ptr;
    }
}

fn main() {
    let mut a = vec![1, 4, 5, 3, 2];
    bubble_sort(&mut a);
    println!("a = {:?}", a);
}
