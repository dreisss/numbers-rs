#[allow(unused)]
pub fn is_palindrome(number: i32) -> bool {
    let number_as_string = number.to_string();
    number_as_string == number_as_string.chars().rev().collect::<String>()
}

#[allow(unused)]
pub fn is_armstrong(number: i32) -> bool {
    let number_as_string = number.to_string();
    let number_length = number_as_string.len() as u32;

    number
        == number_as_string
            .chars()
            .map(|n| n.to_digit(10).unwrap().pow(number_length) as i32)
            .sum()
}

#[cfg(test)]
mod tests {
    use crate::basic::other::*;

    #[test]
    fn test_is_palindrome() {
        assert_eq!(is_palindrome(1), true);
        assert_eq!(is_palindrome(11), true);
        assert_eq!(is_palindrome(1231), false);
        assert_eq!(is_palindrome(1331), true);
        assert_eq!(is_palindrome(13521), false);
    }

    #[test]
    fn test_is_armstrong() {
        assert_eq!(is_armstrong(123), false);
        assert_eq!(is_armstrong(234), false);
        assert_eq!(is_armstrong(371), true);
        assert_eq!(is_armstrong(407), true);
        assert_eq!(is_armstrong(1634), true);
    }
}
