/// Parse a string that encodes a colour
/// and convertsthat to a Rust type
/// Design a data structure RGB
/// Implement RGB Channel trait
/// Implement FromStr for RGB
use std::fmt::Display;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct RGB {
    r: u8,
    g: u8,
    b: u8,
}

trait RGBChannels {
    fn r(&self) -> u8;

    fn g(&self) -> u8;

    fn b(&self) -> u8;
}

#[derive(Debug, PartialEq)]
enum ParseColourError {
    RedOutOfBounds,
    BlueOutOfBounds,
    GreenOutOfBounds,
    Invalid,
}

impl RGBChannels for RGB {
    fn r(&self) -> u8 {
        self.r
    }

    fn g(&self) -> u8 {
        self.g
    }

    fn b(&self) -> u8 {
        self.b
    }
}

impl FromStr for RGB {
    type Err = ParseColourError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(hex) = s.strip_prefix('#') {
            let r = u8::from_str_radix(&hex[0..2], 16)
                .or_else(|_| Err(ParseColourError::RedOutOfBounds))?;
            let g = u8::from_str_radix(&hex[2..4], 16)
                .or_else(|_| Err(ParseColourError::GreenOutOfBounds))?;
            let b = u8::from_str_radix(&hex[4..6], 16)
                .or_else(|_| Err(ParseColourError::BlueOutOfBounds))?;
            Ok(RGB { r, g, b })
        } else {
            // no leading '#'
            Err(ParseColourError::Invalid)
        }
    }
}

impl Display for RGB {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#{:02x}{:02x}{:02x}", self.r(), self.g(), self.b())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_every_colour() {
        let colours = (0_u8..255).zip(0_u8..255).zip(0_u8..255);

        for ((r, g), b) in colours {
            let hex = format!("#{:02x}{:02x}{:02x}", r, g, b);
            let colour: RGB = hex.parse().unwrap();
            assert_eq!(hex, format!("{}", colour));
        }
    }

    #[test]
    #[should_panic]
    fn test_too_short() {
        let _: RGB = "1234".parse().unwrap();
    }

    #[test]
    #[should_panic]
    fn test_not_a_hex_code() {
        let _: RGB = "?".parse().unwrap();
    }

    #[test]
    #[should_panic]
    fn test_invalid_literals() {
        let _: RGB = "?".parse().unwrap();
    }

    #[test]
    #[should_panic]
    fn test_leading_hash_mising() {
        let _: RGB = "aabbcc".parse().unwrap();
    }

    #[test]
    #[should_panic]
    fn test_out_of_bounds() {
        let _: RGB = "00gg00".parse().unwrap();
    }
}
