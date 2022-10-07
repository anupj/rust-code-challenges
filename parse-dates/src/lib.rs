use chrono::{Local, NaiveDate, TimeZone};

fn is_year(field: &str) -> bool {
    field.len() == 4 && field.chars().all(|x| x.is_ascii_digit())
}

/// Parses a string that represents a valid date format.
/// Returns `None` for invalid/unexpected format.
fn flexible_date_parser(text: &str) -> Option<NaiveDate> {
    let text = text.trim();

    // check that there are numbers
    if !text.bytes().any(|x| x.is_ascii_digit()) {
        return None;
    }

    // allow any known delimiter
    let fields: Vec<_> = text.split(['/', '-', '.', ' '].as_slice()).collect();
    let mut year = None;
    let mut month = None;
    let mut day = None;

    for field in fields.iter() {
        if field.len() < 3 {
            continue;
        }

        let m = match &field.to_lowercase()[..3] {
            "jan" => 1,
            "feb" => 2,
            "mar" => 3,
            "apr" => 4,
            "may" => 5,
            "jun" => 6,
            "jul" => 7,
            "aug" => 8,
            "sep" => 9,
            "oct" => 10,
            "nov" => 11,
            "dec" => 12,
            _ => continue,
        };

        month = Some(m)
    }

    for field in fields.iter() {
        if is_year(field) {
            // `ok()` converts a Result<T,E> to an Option<T>
            year = field.parse::<i32>().ok();
            continue;
        }

        if month.is_some() {
            day = field.parse::<u32>().ok();
        } else {
            month = field.parse::<u32>().ok();
        }
    }

    match (year, month, day) {
        (Some(y), Some(m), None) => NaiveDate::from_ymd_opt(y, m, 1),
        (Some(y), Some(m), Some(d)) => NaiveDate::from_ymd_opt(y, m, d),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // let result = add(2, 2);
        // assert_eq!(result, 4);
    }
}
