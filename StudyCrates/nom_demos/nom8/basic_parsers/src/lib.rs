pub mod basic;
pub mod color;
pub mod comment;
pub mod error;
pub mod identifier;
pub mod integers;
pub mod json;
pub mod string;
pub mod whitespace;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
