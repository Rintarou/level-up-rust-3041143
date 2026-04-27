use chrono::NaiveDate;

fn weeks_between(a: &str, b: &str) -> i32 {
    let d_one: NaiveDate = a.parse().unwrap();
    let d_two: NaiveDate = b.parse().unwrap();

    let value = d_two - d_one;
    value.num_weeks().try_into().unwrap_or(0)
}

fn main() {
    let n_weeks = weeks_between("2010-01-21", "2010-10-21");

    println!("hello: {}", n_weeks);
}

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
