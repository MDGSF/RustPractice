fn main() {
    let s = r"mooa
        oano
        otio
        ioon";
    let (mut s1, mut s2) = (String::with_capacity(4), String::with_capacity(4));
    for (idx, val) in s.lines().enumerate() {
        let val = val.trim();
        s1.push_str(val.get(idx..idx + 1).unwrap());
        s2.push_str(val.get((3 - idx)..(3 - idx + 1)).unwrap());

        println!("val = {:?}, s1 = {:?}, s2 = {:?}", val, s1, s2);
    }
    s1.push(' ');
    println!("{:?}", s1 + &s2);
}
