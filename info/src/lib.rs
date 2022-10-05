pub fn info<T: AsRef<str>>(text: &T) {
    println!("{}", text.as_ref());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_str_static_ref() {
        let input = "Rust"; //&'static str
        info(&input);
    }

    #[test]
    fn test_string() {
        let input = "Rust".to_string();
        info(&input);
    }

    #[test]
    fn test_str_ref() {
        let input: &str = "Rust";
        info(&input);
    }
}
