fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

/*
只有实现了 PartialOrd 这个 trait 的泛型才能够比较大小。
只有实现了 Copy trait 的泛型才能在使用赋值 = 的时候执行按位拷贝。
*/
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

/*
泛型 T 如果不实现 Copy trait，也可以用 Clone trait。
Clone trait 则必须显示调用 clone() 函数。
*/
fn largest2<T>(list: &[T]) -> T
where
    T: PartialOrd + Clone,
{
    let mut largest = list[0].clone();

    let mut i = 0;
    while i < list.len() {
        if list[i] > largest {
            largest = list[i].clone();
        }
        i += 1;
    }

    largest
}

/*
也可以修改返回值为对数组中变量的租借，这样的话，根据生命周期自动标注规则，
返回值的生命周期必须和输入的数组一样长。
而 largest1 和 largest2 中的返回值则和输入参数没有任何关系。
*/
fn largest3<T>(list: &[T]) -> &T
where
    T: PartialOrd,
{
    let mut largest = &list[0];

    let mut i = 0;
    while i < list.len() {
        let item = &list[i];
        if item > largest {
            largest = item;
        }
        i += 1;
    }

    largest
}

fn main() {
    println!("Hello, world!");

    let number_list = vec![1, 2, 3, 4, 5];
    let result = largest_i32(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['R', 'u', 's', 't'];
    let result = largest_char(&char_list);
    println!("The largest char is {}", result);

    println!("largest1, The largest number is {}", largest1(&number_list));
    println!("largest1, The largest char is {}", largest1(&char_list));

    println!("largest2, The largest number is {}", largest2(&number_list));
    println!("largest2, The largest char is {}", largest2(&char_list));

    println!("largest3, The largest number is {}", largest3(&number_list));
    println!("largest3, The largest char is {}", largest3(&char_list));
}
