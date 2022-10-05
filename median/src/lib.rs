pub fn median(list: &Vec<f64>) -> Option<f64> {
    // if list is empty return `None`
    if list.is_empty() {
        return None;
    }
    // clone the list because we
    // cannot "mutate" the borrowed `list`
    // vector
    let mut elements = list.clone();
    // Sort the list:
    // Since we are using floating point numbers
    // we can't use the sort() method which requires
    // the type T to implement the `Ord` trait.
    // So we are teaching Rust to compare two floats
    // by passing the logic to compare as a closure
    elements.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let size = elements.len();
    let middle = size / 2;
    // if even then get (mid-1 and mid)/2.0
    if size % 2 == 0 {
        let median_value = (elements[middle - 1] + elements[middle]) / 2.0;
        return Some(median_value);
    } else {
        // if odd then get the middle element
        return Some(elements[middle]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_odd_num_of_elements() {
        let list = vec![1.0, 2.0, 3.0];
        assert_eq!(Some(2.0), median(&list));
    }

    #[test]
    fn test_even_num_of_sorted_elements() {
        let list = vec![1.5, 2.0, 3.0, 8.8];
        assert_eq!(Some(2.5), median(&list));
    }

    #[test]
    fn test_even_num_of_unsorted_elements() {
        let list1 = vec![1.5, 3.0, 5.0, 8.8];
        assert_eq!(Some(4.0), median(&list1));
        let list2: Vec<f64> = vec![
            16.0, 9.0, 12.0, 13.0, 10.0, 13.0, 15.0, 15.0, 13.0, 16.0, 18.0, 22.0, 23.0, 24.0,
            24.0, 25.0,
        ];
        assert_eq!(Some(15.5), median(&list2));
    }

    #[test]
    fn test_empty_list() {
        let list: Vec<f64> = vec![];
        assert_eq!(None, median(&list));
    }
}
