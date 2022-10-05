// simple and direct implementation
pub fn sum_with_missing(numbers: &Vec<Option<i32>>) -> i32 {
    let mut result = 0i32;
    if numbers.len() == 0 {
        return result;
    }
    for i in numbers.iter() {
        match i {
            Some(val) => result += val,
            None => continue,
        }
    }
    result
}

// more functional style implementation
pub fn sum_with_missing_func(numbers: &Vec<Option<i32>>) -> i32 {
    numbers.iter().map(|x| x.unwrap_or(0)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let nn = vec![];
        assert_eq!(sum_with_missing(&nn), 0);
        assert_eq!(sum_with_missing_func(&nn), 0);
    }

    #[test]
    fn no_missing() {
        let nn = vec![Some(1), Some(5), Some(4)];
        assert_eq!(sum_with_missing(&nn), 10);
        assert_eq!(sum_with_missing_func(&nn), 10);
    }

    #[test]
    fn some_missing() {
        let nn = vec![None, Some(1), Some(5), Some(4), None, None];
        assert_eq!(sum_with_missing(&nn), 10);
        assert_eq!(sum_with_missing_func(&nn), 10);
    }

    #[test]
    fn all_missing() {
        let nn = vec![None, None, None];
        assert_eq!(sum_with_missing(&nn), 0);
        assert_eq!(sum_with_missing_func(&nn), 0);
    }
}
