fn main() {
    let s = r"1234
        5678
        9876
        4321";

    let (mut x, mut y) = (0, 0);

    for (idx, val) in s.lines().enumerate() {
        let val = val.trim();

        let left = val.get(idx..idx + 1).unwrap().parse::<u32>().unwrap();
        let right = val
            .get((3 - idx)..(3 - idx + 1))
            .unwrap()
            .parse::<u32>()
            .unwrap();

        println!(
            "idx = {}, val = {}, left = {}, right = {}",
            idx, val, left, right
        );

        x += left;
        y += right;
    }

    assert_eq!(38, x + y);
}
