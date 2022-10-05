// create Temperatur struct
// add to_celsius and to_fahrenheit methods

struct Temperature {
    degrees: f32,
    scale: Scale,
}

enum Scale {
    Celsius,
    Fahrenheit,
}

impl Temperature {
    fn to_celsius(&self) -> f32 {
        match self.scale {
            Scale::Fahrenheit => {
                return (self.degrees - 32.0) * (5.0 / 9.0);
            }
            Scale::Celsius => {
                return self.degrees;
            }
        }
    }

    fn to_fahrenheit(&self) -> f32 {
        match self.scale {
            Scale::Celsius => {
                return (self.degrees * 9.0 / 5.0) + 32.0;
            }
            Scale::Fahrenheit => {
                return self.degrees;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_celsius() {
        let temp_in_fh = Temperature {
            degrees: 90.0,
            scale: Scale::Fahrenheit,
        };
        assert_eq!(32.2, temp_in_fh.to_celsius());
        assert_eq!(90.0, temp_in_fh.to_fahrenheit());
    }

    #[test]
    fn test_to_fahrenheit() {
        let temp_in_fh = Temperature {
            degrees: 40.0,
            scale: Scale::Celsius,
        };
        assert_eq!(40.0, temp_in_fh.to_celsius());
        assert_eq!(104.0, temp_in_fh.to_fahrenheit());
    }
}
