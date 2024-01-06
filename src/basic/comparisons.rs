#[allow(unused)]
pub fn greater_simplest(numbers: Vec<i32>) -> Option<i32> {
    if let Some(mut greater) = numbers.first() {
        for number in &numbers {
            if *number > *greater {
                greater = number;
            }
        }

        return Some(*greater);
    }

    None
}

#[allow(unused)]
pub fn greater_sorting(numbers: &mut Vec<i32>) -> Option<i32> {
    numbers.sort();
    numbers.last().copied()
}

#[allow(unused)]
pub fn smaller_simplest(numbers: Vec<i32>) -> Option<i32> {
    if let Some(mut smaller) = numbers.first() {
        for number in &numbers {
            if *number < *smaller {
                smaller = number;
            }
        }

        return Some(*smaller);
    }

    None
}

#[allow(unused)]
pub fn smaller_sorting(numbers: &mut Vec<i32>) -> Option<i32> {
    numbers.sort();
    numbers.first().copied()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greater_simplest() {
        assert_eq!(greater_simplest(vec![0, 1, 2]), Some(2));
        assert_eq!(greater_simplest(vec![2, 1, 3, 4]), Some(4));
        assert_eq!(greater_simplest(vec![2, 2, 1, 3, 4]), Some(4));
        assert_eq!(greater_simplest(vec![5, 2, 1, 3, 4, 3]), Some(5));
    }

    #[test]
    fn test_greater_sorting() {
        assert_eq!(greater_sorting(&mut vec![0, 1, 2]), Some(2));
        assert_eq!(greater_sorting(&mut vec![2, 1, 3, 4]), Some(4));
        assert_eq!(greater_sorting(&mut vec![2, 2, 1, 3, 4]), Some(4));
        assert_eq!(greater_sorting(&mut vec![5, 2, 1, 3, 4, 3]), Some(5));
    }

    #[test]
    fn test_smaller_simplest() {
        assert_eq!(smaller_simplest(vec![0, 1, 2]), Some(0));
        assert_eq!(smaller_simplest(vec![2, 1, 3, 4]), Some(1));
        assert_eq!(smaller_simplest(vec![2, 2, 1, 3, 4]), Some(1));
        assert_eq!(smaller_simplest(vec![5, 2, 1, 3, 4, 3]), Some(1));
    }

    #[test]
    fn test_smaller_sorting() {
        assert_eq!(smaller_sorting(&mut vec![0, 1, 2]), Some(0));
        assert_eq!(smaller_sorting(&mut vec![2, 1, 3, 4]), Some(1));
        assert_eq!(smaller_sorting(&mut vec![2, 2, 1, 3, 4]), Some(1));
        assert_eq!(smaller_sorting(&mut vec![5, 2, 1, 3, 4, 3]), Some(1));
    }
}
