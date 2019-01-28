use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    assert!(scores.is_empty());

    // 插入
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    scores.insert(String::from("eye"), 5000);

    println!("scores.len() = {}", scores.len());

    // 遍历
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
    println!();

    // 判断是否存在
    let blue_exist = scores.contains_key(&String::from("Blue"));
    println!("blue_exist = {}", blue_exist);

    let apple_exist = scores.contains_key(&String::from("apple"));
    println!("apple_exist = {}", apple_exist);

    // 删除
    scores.remove(&String::from("eye"));
    scores.remove(&String::from("eye"));
    println!("remove eye: {:?}", scores);

    // 获取,不能修改
    let team_name = String::from("Blue");
    if let Some(score) = scores.get(&team_name) {
        println!("get blue score = {:?}", score);
    } else {
        println!("Blue not exists");
    }

    // 获取，可以修改
    let team_name = String::from("Blue");
    if let Some(score) = scores.get_mut(&team_name) {
        println!("get mut blue score = {:?}, add 20.", score);
        *score += 20;
    } else {
        println!("Blue not exists");
    }
    println!("new blue score: {:?}", scores);

    // 覆盖已有的值
    scores.insert(String::from("Blue"), 255);
    println!("insert new blue score: {:?}", scores);

    // 只有在键没有对应的值时插入
    scores.entry(String::from("Yellow")).or_insert(1111);
    scores.entry(String::from("red")).or_insert(2222);
    println!("{:?}", scores);

    test1();
    test2();
    test3();
    test4();
}

fn test1() {
    // string 的所有权移动到了 map 内部
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // println!("field_name = {}", field_name);
}

// 计算字符串中每个单词出现的次数
fn test2() {
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("test2> {:?}", map);
}

// 计算字符串中每个字符出现的次数
fn test3() {
    let mut letters = HashMap::new();

    for ch in "a short treatise on fungi".chars() {
        let counter = letters.entry(ch).or_insert(0);
        *counter += 1;
    }

    assert_eq!(letters[&'s'], 2);
    assert_eq!(letters[&'t'], 3);
    assert_eq!(letters[&'u'], 1);
    assert_eq!(letters.get(&'y'), None);
    println!("test3> {:?}", letters);
}

// 根据数组来构建 HashMap
fn test4() {
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
    println!("test4> {:?}", scores);
}
