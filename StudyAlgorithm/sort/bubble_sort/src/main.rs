fn main() {
    let mut a = vec![3, 2, 1, 4, 5, 6];
    bubble_sort(&mut a);
    println!("a = {:?}", a);
}

fn bubble_sort<T>(v: &mut [T])
where
    T: PartialOrd + Copy,
{
    let mut i = 0;
    while i < v.len() {
        let mut j = v.len() - 1;
        while j > i {
            if v[j - 1] > v[j] {
                v.swap(j - 1, j);
            }
            j -= 1;
        }
        i += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bubble_sort1() {
        let mut a = vec![3, 2, 1, 4, 5, 6];
        bubble_sort(&mut a);
        assert_eq!(&a[..], &[1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn test_bubble_sort2() {
        let mut a = vec![1, 2, 3, 4, 5, 6];
        bubble_sort(&mut a);
        assert_eq!(&a[..], &[1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn test_bubble_sort3() {
        let mut a = vec![1, 1, 1];
        bubble_sort(&mut a);
        assert_eq!(&a[..], &[1, 1, 1]);
    }

    #[test]
    fn test_bubble_sort4() {
        let mut a: Vec<i32> = vec![];
        bubble_sort(&mut a);
        assert_eq!(&a[..], &[]);
    }

    #[test]
    fn test_bubble_sort5() {
        let mut a = vec!['r', 'u', 's', 't'];
        bubble_sort(&mut a);
        assert_eq!(&a[..], &['r', 's', 't', 'u']);
    }
}
