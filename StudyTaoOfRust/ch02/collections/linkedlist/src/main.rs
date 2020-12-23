use std::collections::LinkedList;

fn main() {
    let mut list1 = LinkedList::new();

    list1.push_back('a');

    let mut list2 = LinkedList::new();
    list2.push_back('b');
    list2.push_back('c');

    list1.append(&mut list2);
    println!("list1 = {:?}", list1);
    println!("list2 = {:?}", list2);

    list1.pop_front();
    println!("list1 = {:?}", list1);

    list1.push_front('e');
    println!("list1 = {:?}", list1);

    list2.push_front('f');
    println!("list2 = {:?}", list2);
}
