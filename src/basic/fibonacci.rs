#[allow(unused)]
pub fn nth_term_recursive(n: u32) -> u32 {
    if n <= 2 {
        1
    } else {
        nth_term_recursive(n - 1) + nth_term_recursive(n - 2)
    }
}

#[allow(unused)]
pub fn nth_term_iterative(n: u32) -> u32 {
    if n <= 2 {
        1
    } else {
        let (mut previous, mut current) = (1, 2);

        for a in 3..n {
            current += previous;
            previous = current - previous;
        }

        current
    }
}

#[cfg(test)]
mod tests {
    use crate::basic::fibonacci::*;

    #[test]
    fn test_nth_term_recursive() {
        assert_eq!(nth_term_recursive(1), 1);
        assert_eq!(nth_term_recursive(5), 5);
        assert_eq!(nth_term_recursive(10), 55);
        assert_eq!(nth_term_recursive(20), 6765);
        assert_eq!(nth_term_recursive(30), 832040);
        assert_eq!(nth_term_iterative(40), 102334155);
    }

    #[test]
    fn test_nth_term_iterative() {
        assert_eq!(nth_term_iterative(1), 1);
        assert_eq!(nth_term_iterative(5), 5);
        assert_eq!(nth_term_iterative(10), 55);
        assert_eq!(nth_term_iterative(20), 6765);
        assert_eq!(nth_term_iterative(30), 832040);
        assert_eq!(nth_term_iterative(40), 102334155);
    }
}
