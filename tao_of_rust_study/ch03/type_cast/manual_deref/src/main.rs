use std::borrow::Borrow;
use std::ops::Deref;
use std::rc::Rc;

fn main() {
    let x = Rc::new("hello");
    let y = x.clone(); // std::rc::Rc<&str>
    let z = (*x).clone(); // str
    println!("x = {}", x);
    println!("y = {}", y);
    println!("z = {}", z);

    match &x[..] {
        "hello" => println!("hello"),
        _ => {}
    }
}

/*
Base usage: match匹配里需要手动解引用

将match &x改为以下5种形式任意一种即可:

match x.deref()，直接调用deref方法，需要use std::ops::Deref。
match x.as_ref()，String类型提供了as_ref方法来返回一个&str类型，该方法定义于AsRef trait中。
match x.borrow()，方法borrow定义于Borrow trait中，行为和AsRef类型。需要use std::borrow::Borrow。
match &*x，使用“解引用”操作符，将String转换为str，然后再用“引用”操作符转为&str。
match &x[..]，这是因为String类型的index操作可以返回&str类型。
*/
