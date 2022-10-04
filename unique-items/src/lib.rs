use std::collections::HashSet;

// Filters duplicates within a list.
// Accepts a Vec<i32> reference and returns a
// Vec<i32> with no duplicates.
pub fn unique(list: &Vec<i32>) -> Vec<i32> {
    let mut unique_set = HashSet::new();
    // inserting values into a set
    // will ensure that its unique
    for i in list {
        unique_set.insert(*i);
    }
    // convert the set back into a list
    // let return_list: Vec<_> = unique_set.into_iter().collect();
    let return_list: Vec<_> = Vec::from_iter(unique_set);

    return return_list;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_with_no_duplicates() {
        let list = vec![1, 4, 5];
        assert_eq!(list, unique(&list));
    }

    #[test]
    fn test_with_duplicates() {
        let list = vec![1, 1, 3];
        assert_eq!(vec![1, 3], unique(&list));
    }
}
