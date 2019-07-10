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

fn largest2<T>(list: &[T]) -> T
where
    T: PartialOrd + Clone,
{
    let mut largest = list[0].clone();

    let mut i = 0;
    while i < list.len() {
        let item = list[i].clone();
        if item > largest {
            largest = item.clone();
        }
        i += 1;
    }

    largest
}

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
