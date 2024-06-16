fn anagram03(s1: &str, s2: &str) -> bool {
    if s1.len() != s2.len() {
        return false;
    }

    let mut alist = Vec::new();
    let mut blist = Vec::new();
    for c in s1.chars() {
        alist.push(c);
    }
    for c in s2.chars() {
        blist.push(c);
    }
    alist.sort();
    blist.sort();

    let mut pos: usize = 0;
    while pos < alist.len() {
        if alist[pos] != blist[pos] {
            return false;
        }
        pos += 1;
    }

    true
}

fn main() {
    let s1 = "rust";
    let s2 = "trus";
    let result = anagram03(s1, s2);
    println!("s1 and s2 is anagram: {result}");
}
