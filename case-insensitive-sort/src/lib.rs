// users should be sorted in-place
// Accepts all unicode characters
pub fn sort_usernames(usernames: &mut Vec<&str>) {
    if usernames.is_empty() {
        return;
    }
    // use sort_by to instruct Rust
    // on how to compare 2 values
    usernames.sort_by(|a, b| a.to_lowercase().partial_cmp(&b.to_lowercase()).unwrap());
}

// Same as above but uses generics and trait bounds
pub fn generic_sort_usernames<T: AsRef<str>>(usernames: &mut Vec<T>) {
    if usernames.is_empty() {
        return;
    }
    // use sort_by to instruct Rust
    // on how to compare 2 values
    usernames.sort_by(|a, b| {
        a.as_ref()
            .to_lowercase()
            .partial_cmp(&b.as_ref().to_lowercase())
            .unwrap()
    });
}

// Same as above but uses generics and trait bounds
// Uses sort_by_cached_key instead
pub fn generic_sort_usernames_key<T: AsRef<str>>(usernames: &mut Vec<T>) {
    if usernames.is_empty() {
        return;
    }
    // Sorts the slice indirectly, unsing a key
    // extraction function and caching the keys
    usernames.sort_by_cached_key(|x| x.as_ref().to_lowercase());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_usernames() {
        let mut users = vec!["Todd", "bella", "amy"];
        sort_usernames(&mut users);
        assert_eq!(vec!["amy", "bella", "Todd"], users);
    }

    #[test]
    fn test_generic_sort_usernames() {
        let mut users = vec!["Todd", "Amy", "mike99", "Jennifer", "alison"];
        generic_sort_usernames(&mut users);
        assert_eq!(vec!["alison", "Amy", "Jennifer", "mike99", "Todd"], users);
    }

    #[test]
    fn test_generic_sort_by_cached_key() {
        let mut users = vec!["Todd", "Amy", "mike99", "Jennifer", "alison"];
        generic_sort_usernames_key(&mut users);
        assert_eq!(vec!["alison", "Amy", "Jennifer", "mike99", "Todd"], users);
    }
}
