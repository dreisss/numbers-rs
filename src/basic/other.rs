#[allow(unused)]
pub fn is_palindrome(number: u32) -> bool {
    let number_as_string = number.to_string();
    number_as_string == number_as_string.chars().rev().collect::<String>()
}

#[allow(unused)]
pub fn is_armstrong(number: u32) -> bool {
    let number_as_string = number.to_string();
    let number_length = number_as_string.len() as u32;

    number
        == number_as_string
            .chars()
            .map(|n| n.to_digit(10).unwrap().pow(number_length))
            .sum()
}

#[allow(unused)]
pub fn sum_digits(number: u32) -> u32 {
    number
        .to_string()
        .chars()
        .map(|d| d.to_digit(10).unwrap())
        .sum()
}

#[allow(unused)]
pub fn reverse(number: u32) -> u32 {
    number
        .to_string()
        .chars()
        .rev()
        .collect::<String>()
        .parse()
        .unwrap()
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

    #[test]
    fn test_sum_digits() {
        assert_eq!(sum_digits(123), 6);
        assert_eq!(sum_digits(234), 9);
        assert_eq!(sum_digits(371), 11);
        assert_eq!(sum_digits(407), 11);
        assert_eq!(sum_digits(1634), 14);
    }

    #[test]
    fn test_reverse() {
        assert_eq!(reverse(123), 321);
        assert_eq!(reverse(234), 432);
        assert_eq!(reverse(371), 173);
        assert_eq!(reverse(407), 704);
        assert_eq!(reverse(1634), 4361);
    }
}
