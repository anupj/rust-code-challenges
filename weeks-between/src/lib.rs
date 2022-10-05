use chrono::NaiveDate;
/// Assume that date1 and date2 are
/// always valid dates with the following
/// format YYYY-MM-DD
/// When date2 occurs before date1 return a
/// a negative number
pub fn weeks_between(date1: &str, date2: &str) -> i32 {
    let t1 = NaiveDate::parse_from_str(date1, "%Y-%m-%d").unwrap();
    let t2 = NaiveDate::parse_from_str(date2, "%Y-%m-%d").unwrap();

    let n_weeks = (t2 - t1).num_days() / 7;
    n_weeks as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn same_day() {
        let n_weeks = weeks_between("1010-10-10", "1010-10-10");
        assert_eq!(n_weeks, 0);
    }

    #[test]
    fn one_week() {
        let n_weeks = weeks_between("1010-10-10", "1010-10-18");
        assert_eq!(n_weeks, 1);
    }

    #[test]
    fn past() {
        let n_weeks = weeks_between("1010-10-18", "1010-10-10");
        assert_eq!(n_weeks, -1);
    }
}
