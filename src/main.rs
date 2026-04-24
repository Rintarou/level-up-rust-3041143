use std::cmp::Ordering;

use chrono::{Duration, TimeZone};
use chrono::{Local, NaiveDate};

struct ImportantEvent {
    what: String,
    when: NaiveDate,
}

trait Deadline {
    fn is_passed(&self) -> bool;
}

impl Deadline for ImportantEvent {
    fn is_passed(&self) -> bool {
        self.when < Local::now().date_naive()
    }
}

fn main() {
    let missed_christmas = ImportantEvent {
        what: String::from("Christmas"),
        when: NaiveDate::from_ymd_opt(2020, 12, 25).unwrap(),
    };

    if missed_christmas.is_passed() {
        println!("oh well, maybe next year");
    } else {
        println!("☃︎");
    }
}

#[test]
fn in_past() {
    let event = ImportantEvent {
        what: String::from("friend's birthday"),
        when: Local::now().date_naive() - Duration::hours(25),
    };

    assert!(event.is_passed())
}

#[test]
fn in_future() {
    let event = ImportantEvent {
        what: String::from("friend's birthday"),
        when: Local::now().date_naive() + Duration::hours(25),
    };

    assert!(!event.is_passed())
}
