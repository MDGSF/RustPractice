fn main() {
    let s = [0, 1, 1, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55];
    assert_eq!(s.binary_search(&13), Ok(9));
    assert_eq!(s.binary_search(&4), Err(7));

    let r = s.binary_search(&1);
    assert!(match r {
        Ok(1..=4) => true,
        _ => false,
    });

    let seek = 13;
    assert_eq!(s.binary_search_by(|probe| probe.cmp(&seek)), Ok(9));

    let s = [
        (0, 0),
        (2, 1),
        (4, 1),
        (5, 1),
        (3, 1),
        (1, 2),
        (2, 3),
        (4, 5),
        (5, 8),
        (3, 13),
        (1, 21),
        (2, 34),
        (4, 55),
    ];
    assert_eq!(s.binary_search_by_key(&13, |&(a, b)| b), Ok(9));
}
