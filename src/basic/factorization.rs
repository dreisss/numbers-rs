#[allow(unused)]
pub fn factors(number: u32) -> Vec<u32> {
    let mut factors = vec![];

    for n in 1..=number {
        if number % n == 0 {
            factors.push(n);
        }
    }

    factors
}

#[allow(unused)]
pub fn is_perfect(number: u32) -> bool {
    number == factors(number).into_iter().sum::<u32>() - number
}

#[cfg(test)]
mod tests {
    use crate::basic::factorization::*;

    #[test]
    fn test_factors() {
        assert_eq!(factors(1), vec![1]);
        assert_eq!(factors(2), vec![1, 2]);
        assert_eq!(factors(3), vec![1, 3]);
        assert_eq!(factors(6), vec![1, 2, 3, 6]);
        assert_eq!(factors(12), vec![1, 2, 3, 4, 6, 12]);
    }

    #[test]
    fn test_is_perfect() {
        assert_eq!(is_perfect(6), true);
        assert_eq!(is_perfect(25), false);
        assert_eq!(is_perfect(28), true);
        assert_eq!(is_perfect(496), true);
        assert_eq!(is_perfect(8128), true);
    }
}
