/// Encode  and decode string of characters
/// e.g. LinkedIn => 1L1i1k1e1d1I1n
/// e.g. AAAAAaAA => 5A1a2A
/// Extra credit
///  - long run & arbitrary byte streams

pub fn run_length_encode(input: &str) -> String {
    let mut encoded_str = String::new();
    if input.is_empty() {
        return encoded_str;
    }
    let mut char_count = 1u32;
    let mut prev_char = input.chars().next().unwrap();

    for (i, c) in input.chars().enumerate() {
        if i == 0 {
            println!("i is {}", i);
            continue;
        }
        if c == prev_char {
            char_count += 1;
        } else {
            encoded_str.push_str(&char_count.to_string());
            encoded_str.push_str(&prev_char.to_string());
            prev_char = c;
            char_count = 1;
        }
    }
    encoded_str.push_str(&char_count.to_string());
    encoded_str.push_str(&prev_char.to_string());

    encoded_str
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_non_repeating_string() {
        assert_eq!(
            "1L1i1n1k1e1d1I1n".to_string(),
            run_length_encode("LinkedIn")
        );
    }

    #[test]
    fn test_repeating_string() {
        assert_eq!("5A1a2A".to_string(), run_length_encode("AAAAAaAA"));
    }
}
