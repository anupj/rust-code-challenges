/// Encode  and decode string of characters
/// e.g. LinkedIn => 1L1i1k1e1d1I1n
/// e.g. AAAAAaAA => 5A1a2A
/// Extra credit
///  - long run & arbitrary byte streams

mod run_length_encoding {
    pub fn encode(input: &str) -> String {
        if input.is_empty() {
            return String::with_capacity(0);
        }
        let mut encoded_str = String::with_capacity(input.len() / 2);
        // Starting with 1 because we know
        // atleast 1 char will be encountered
        let mut char_count = 1u32;
        // setting prev_char to be the first char
        // in the input string
        let mut prev_char = input.chars().next().unwrap();

        for (i, c) in input.chars().enumerate() {
            if i == 0 {
                // skip the loop
                // if its the first char
                continue;
            }
            if c == prev_char && char_count < 9 {
                // keep going if the current char
                // is the same as previous char
                // until we encounter 9 such chars
                char_count += 1;
            } else {
                encoded_str.push_str(&format!("{}{}", char_count, prev_char));
                prev_char = c;
                char_count = 1;
            }
        }
        encoded_str.push_str(&format!("{}{}", char_count, prev_char));

        encoded_str
    }

    pub fn decode(input: &str) -> String {
        if input.is_empty() {
            return String::with_capacity(0);
        }
        let mut result = String::with_capacity(input.len() * 2);
        let mut chars = input.chars();

        while let (Some(n), Some(c)) = (chars.next(), chars.next()) {
            // Convert from char to u32
            let n = n.to_digit(10).unwrap();
            for _ in 0..n {
                result.push(c);
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::run_length_encoding::*;

    #[test]
    fn test_non_repeating_string() {
        assert_eq!("1L1i1n1k1e1d1I1n".to_string(), encode("LinkedIn"));
    }

    #[test]
    fn test_repeating_string() {
        assert_eq!("5A1a2A".to_string(), encode("AAAAAaAA"));
    }

    #[test]
    fn test_abc() {
        let input = "abc";
        assert_eq!(encode(input), "1a1b1c");
        assert_eq!(decode(&encode(input)), input);
    }

    #[test]
    fn test_round_trip() {
        let input = "LinkedIn";
        println!("encoded input is: {}", encode(&input));
        assert_eq!(decode(&encode(input)), input);
    }

    #[test]
    fn test_long_run() {
        let input = "AAAAA AAAAAAAAAA AAAAAAAAAAAAAAAAAAAA";
        assert_eq!(encode(&input), "5A1 9A1A1 9A9A2A");
        assert_eq!(decode(&encode(input)), input);
    }
}
