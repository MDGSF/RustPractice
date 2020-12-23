use rust_msgpack::binary;

pub const HEXTABLE: &str = "0123456789abcdef";

pub fn hexdump(data: &[u8]) -> String {
    let mut result: Vec<u8> = Vec::new();

    let mut buf1: Vec<u8> = vec![0; 14];
    let mut buf2: Vec<u8> = vec![0; 14];
    let mut right_chars: Vec<u8> = vec![0; 18];
    let mut used = 0;
    let mut n = 0;
    for (i, c) in data.iter().enumerate() {
        if used == 0 {
            binary::BigEndian::put_uint32(&mut buf1[0..4], n);
            hexencode(&mut buf2[0..8], &buf1[0..4]);
            buf2[8] = b' ';
            buf2[9] = b' ';
            result.extend(&buf2[0..10]);
        }
        hexencode(&mut buf2[..], &data[i..i + 1]);
        buf2[2] = b' ';
        let mut l = 3;
        if used == 7 {
            buf2[3] = b' ';
            l = 4;
        } else if used == 15 {
            buf2[3] = b' ';
            buf2[4] = b'|';
            l = 5;
        }
        result.extend(&buf2[0..l]);

        right_chars[used] = tochar(*c);
        n += 1;
        used += 1;

        if used == 16 {
            right_chars[16] = b'|';
            right_chars[17] = b'\n';
            result.extend(&right_chars[..]);
            used = 0;
        }
    }

    String::from_utf8(result).unwrap()
}

pub fn hexencode(dst: &mut [u8], src: &[u8]) -> i32 {
    for (i, v) in src.iter().enumerate() {
        dst[i * 2] = HEXTABLE.as_bytes()[(v >> 4) as usize];
        dst[i * 2 + 1] = HEXTABLE.as_bytes()[(v & 0x0f) as usize];
    }
    (src.len() * 2) as i32
}

fn tochar(b: u8) -> u8 {
    if b < 32 || b > 126 {
        return b'.';
    }
    b
}
