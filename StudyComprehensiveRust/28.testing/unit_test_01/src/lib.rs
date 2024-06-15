pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn first_world(text: &str) -> &str {
    match text.find(' ') {
        Some(idx) => &text[..idx],
        None => &text,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_empty() {
        assert_eq!(first_world(""), "");
    }

    #[test]
    fn test_single_word() {
        assert_eq!(first_world("Hello"), "Hello");
    }

    #[test]
    fn test_multiple_words() {
        assert_eq!(first_world("Hello World"), "Hello");
    }
}
