fn main() {
    println!("Hello, world!");

    println!("1 + 2 = {}", 1u32 + 2);

    println!("1 - 2 = {}", 1i32 - 2);

    println!("true AND false is {}", true && false);
    println!("true OR false is {}", true || false);
    println!("Not true is {}", !true);

    println!("0b0011u32 & 0b0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0b0011u32 | 0b0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0b0011u32 ^ 0b0101 is {:04b}", 0b0011u32 ^ 0b0101);

    println!("1u32 << 5 is {}", 1u32 << 5);
    println!("0x80u32 >> 2 is 0x{:x}", 0x80u32 >> 2);

    println!("1_000_000u32 is {}", 1_000_000u32)
}
