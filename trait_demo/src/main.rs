// T 是一个泛型
// PartialOrd 是一个 trait
// Copy 也是一个 trait
// <T: PartialOrd + Copy> 意思是：实现了 PartialOrd 和 Copy 这两个 trait 的泛型。
// 也就是说传递到函数 largest 中的 T 必须同时实现了接口 PartialOrd 和 Copy。
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// 和 largest 函数是一样的, where 只是语法糖
fn largest1<T>(list: &[T]) -> T
where
    T: PartialOrd + Copy,
{
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest1(&char_list);
    println!("The largest char is {}", result);
}
