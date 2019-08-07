/*
 * Cell 和 RefCell 是一种外部不可变但内部可变的容器
 * Cell 用于 Copy 类型
 * RefCell 用于非 Copy 类型
 */

use std::cell::Cell;

struct Foo {
    x: u32,
    y: Cell<u32>,
}

fn main() {
    let foo = Foo {
        x: 1,
        y: Cell::new(3),
    };
    assert_eq!(1, foo.x);
    assert_eq!(3, foo.y.get());
    foo.y.set(5);
    assert_eq!(5, foo.y.get());
}
