#[allow(unused)]
fn is_prime(number: i32) -> bool {
    if number <= 1 {
        false
    } else {
        for n in 2..=(number as f32).sqrt() as i32 {
            if number % n == 0 {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use crate::basic::primes::*;

    #[test]
    fn test_is_prime() {
        assert_eq!(is_prime(1), false);
        assert_eq!(is_prime(2), true);
        assert_eq!(is_prime(3), true);
        assert_eq!(is_prime(4), false);
        assert_eq!(is_prime(5), true);
        assert_eq!(is_prime(67), true);
        assert_eq!(is_prime(1000), false);
        assert_eq!(is_prime(2137), true);
    }
}
