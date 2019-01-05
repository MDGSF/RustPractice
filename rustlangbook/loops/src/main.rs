fn main() {
    test5();
}

fn test5() {
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}

//fn test4() {
//    let a = [10, 20, 30, 40, 50];
//
//    for element in a.iter() {
//        println!("the value is: {}", element);
//    }
//}

//fn test3() {
//    let a = [10, 20, 30, 40, 50];
//    let mut index = 0;
//
//    while index < 5 {
//        println!("the value is: {}", a[index]);
//        index += 1;
//    }
//}

//fn test2() {
//    let mut number = 3;
//
//    while number != 0 {
//        println!("{}!", number);
//
//        number -= 1;
//    }
//
//    println!("LIFTOFF!!!");
//}

//fn test1() {
//    let mut counter = 0;
//
//    let result = loop {
//        counter += 1;
//
//        if counter == 10 {
//            break counter * 2;
//        }
//    };
//
//    assert_eq!(result, 20);
//}
