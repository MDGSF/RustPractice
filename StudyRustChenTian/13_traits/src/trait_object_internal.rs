use std::fmt::{Debug, Display};
use std::mem::transmute;

fn main() {
    let s = String::from("hello world!");
    let s1 = String::from("goodbye world!");

    let w1: &dyn Display = &s;
    let w2: &dyn Debug = &s;

    let w3: &dyn Display = &s1;
    let w4: &dyn Debug = &s1;

    let (addr1, vtable1): (usize, usize) = unsafe { transmute(w1) };
    let (addr2, vtable2): (usize, usize) = unsafe { transmute(w2) };
    let (addr3, vtable3): (usize, usize) = unsafe { transmute(w3) };
    let (addr4, vtable4): (usize, usize) = unsafe { transmute(w4) };

    println!(
        "s: {:p}, s1: {:p}, main(): {:p}",
        &s, &s1, main as *const ()
    );

    println!("addr1: 0x{:x}, vtable1: 0x{:x}", addr1, vtable1);
    println!("addr2: 0x{:x}, vtable2: 0x{:x}", addr2, vtable2);
    println!("addr3: 0x{:x}, vtable3: 0x{:x}", addr3, vtable3);
    println!("addr4: 0x{:x}, vtable4: 0x{:x}", addr4, vtable4);

    assert_eq!(addr1, addr2);
    assert_eq!(addr3, addr4);

    assert_eq!(vtable1, vtable3);
    assert_eq!(vtable2, vtable4);
}
