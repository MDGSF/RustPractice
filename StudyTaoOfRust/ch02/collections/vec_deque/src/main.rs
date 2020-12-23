use std::collections::VecDeque;

fn main() {
    let mut buf = VecDeque::new();
    buf.push_front(1);
    buf.push_front(2);
    println!("{:?}", buf);

    for i in 0..10 {
        buf.push_front(i);
    }
    println!("{:?}", buf);

    buf.push_back(3);
    buf.push_back(4);
    buf.push_back(5);
    println!("{:?}", buf);

    assert_eq!(buf.get(0), Some(&9));
}
