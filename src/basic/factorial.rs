#[allow(unused)]
pub fn factorial_recursive(number: i32) -> i32 {
    if number <= 1 {
        1
    } else {
        number * factorial_recursive(number - 1)
    }
}

#[allow(unused)]
pub fn factorial_iterative(number: i32) -> i32 {
    let mut factorial = 1;

    for n in 2..=number {
        factorial *= n;
    }

    factorial
}

#[cfg(test)]
mod tests {
    use crate::basic::factorial::*;

    #[test]
    fn test_factorial_recursive() {
        assert_eq!(factorial_recursive(0), 1);
        assert_eq!(factorial_recursive(1), 1);
        assert_eq!(factorial_recursive(2), 2);
        assert_eq!(factorial_recursive(3), 6);
        assert_eq!(factorial_recursive(4), 24);
        assert_eq!(factorial_recursive(5), 120);
        assert_eq!(factorial_recursive(10), 3628800);
    }

    #[test]
    fn test_factorial_iterative() {
        assert_eq!(factorial_iterative(0), 1);
        assert_eq!(factorial_iterative(1), 1);
        assert_eq!(factorial_iterative(2), 2);
        assert_eq!(factorial_iterative(3), 6);
        assert_eq!(factorial_iterative(4), 24);
        assert_eq!(factorial_iterative(5), 120);
        assert_eq!(factorial_iterative(10), 3628800);
    }
}
