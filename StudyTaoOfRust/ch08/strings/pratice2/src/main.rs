fn main() {
    let s = r"mooa
        oano
        otio
        ioon";
    let v = s.split('\n').collect::<Vec<_>>();
    let mut s1 = String::new();
    let mut s2 = String::new();
    for (idx, val) in v.iter().enumerate() {
        let x = val.trim();
        let y = x.chars().collect::<Vec<_>>();
        println!("{:?}", y);
        s1.push(y[idx]);
        s2.push(y[3 - idx]);
    }
    s1.push(' ');
    println!("{:?}", s1 + &s2);
}
