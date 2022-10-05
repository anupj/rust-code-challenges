use chrono::{Date, Local, TimeZone};

struct ImportantEvent {
    what: String,
    when: Date<Local>,
}

trait Deadline {
    fn is_passed(&self) -> bool;
}

impl Deadline for ImportantEvent {
    fn is_passed(&self) -> bool {
        self.when < Local::today()
    }
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;

    #[test]
    fn test_in_past() {
        let event = ImportantEvent {
            what: String::from("friend's birthday"),
            when: Local::today() - Duration::hours(25),
        };

        assert!(event.is_passed());
    }

    #[test]
    fn test_in_future() {
        let event = ImportantEvent {
            what: String::from("friend's birthday"),
            when: Local::today() + Duration::hours(25),
        };

        assert!(!event.is_passed());
    }
}
