use chrono::NaiveDate;

#[derive(Debug, Clone)]
struct WeeksBetweenError;

fn weeks_between(a: &str, b: &str) -> Result<i32, WeeksBetweenError> {
    let d_one: NaiveDate = a.parse().map_err(|_| WeeksBetweenError)?;
    let d_two: NaiveDate = b.parse().map_err(|_| WeeksBetweenError)?;

    let value = d_two - d_one;
    value.num_weeks().try_into().map_err(|_| WeeksBetweenError)
}

fn main() {
    let n_weeks = weeks_between("2010-01-21", "2010-10-21").unwrap();
    println!("hello: {}", n_weeks);
}

#[test]
fn same_day() {
    let n_weeks = weeks_between("1010-10-10", "1010-10-10").unwrap();
    assert_eq!(n_weeks, 0);
}

#[test]
fn one_week() {
    let n_weeks = weeks_between("1010-10-10", "1010-10-18").unwrap();
    assert_eq!(n_weeks, 1);
}

#[test]
fn past() {
    let n_weeks = weeks_between("1010-10-18", "1010-10-10").unwrap();
    assert_eq!(n_weeks, -1);
}
