// Filters duplicates within a list.
// Accepts a Vec<i32> reference and returns a
// Vec<i32> with no duplicates.
pub fn unique(list: &Vec<i32>) -> Vec<i32> {
    // if empty return empty vector
    if list.len() == 0 {
        return vec![];
    }
    // clone into a result list
    let mut result_list = list.clone();
    // sort it
    result_list.sort();
    // dedupe it
    result_list.dedup();
    result_list
}

// Generic version of the above function
fn generic_unique<T: Ord + Clone>(list: &Vec<T>) -> Vec<T> {
    // if empty return empty vector
    if list.len() == 0 {
        return vec![] as Vec<T>;
    }
    let mut result_list = list.clone();
    result_list.sort();
    result_list.dedup();
    result_list
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_empty_list() {
        let list = vec![];
        assert_eq!(list, unique(&list));
        let list2: Vec<String> = vec![];
        assert_eq!(vec![] as Vec<String>, generic_unique(&list2));
    }

    #[test]
    fn test_sorted_with_no_duplicates() {
        let list = vec![1, 4, 5];
        assert_eq!(list, unique(&list));
        let list2: Vec<&str> = vec!["abc", "def", "hij"];
        assert_eq!(list2, generic_unique(&list2));
    }

    #[test]
    fn test_sorted_list_with_duplicates() {
        let list = vec![1, 1, 3];
        assert_eq!(vec![1, 3], unique(&list));
        let list2: Vec<&str> = vec!["abc", "abc", "def", "hij"];
        assert_eq!(vec!["abc", "def", "hij"], generic_unique(&list2));
    }

    #[test]
    fn test_unsorted_with_no_duplicates() {
        let list = vec![4, 1, 5];
        assert_eq!(vec![1, 4, 5], unique(&list));
        let list2: Vec<&str> = vec!["abc", "hij", "def"];
        assert_eq!(vec!["abc", "def", "hij"], generic_unique(&list2));
    }

    #[test]
    fn test_unsorted_list_with_duplicates() {
        let list = vec![3, 3, 1];
        assert_eq!(vec![1, 3], unique(&list));
        let list2: Vec<&str> = vec!["abc", "abc", "hij", "xyz", "def", "def"];
        assert_eq!(vec!["abc", "def", "hij", "xyz"], generic_unique(&list2));
    }
}
