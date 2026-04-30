use std::{num::ParseIntError, str::FromStr, string::ParseError};

struct Isbn {
    raw: String,
    digits: Vec<u8>,
}

impl FromStr for Isbn {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let vex = s
            .chars()
            .filter_map(|c| c.to_digit(10))
            .map(|digit| digit.to_be_bytes()[0])
            .collect::<Vec<u8>>();
        Ok(Isbn {
            digits: vex,
            raw: s.to_string(),
        })
    }
}

impl Isbn {
    fn check(&self) -> bool {
        match &self.digits.last() {
            Some(check_digit) => calculate_check_digit(&self.digits).eq(check_digit),
            None => false,
        }
    }
}

impl std::fmt::Display for Isbn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.raw)
    }
}

// https://en.wikipedia.org/wiki/International_Standard_Book_Number#ISBN-13_check_digit_calculation
fn calculate_check_digit(digits: &[u8]) -> u8 {
    let coefficient = [1u8, 3].iter().cycle().take(12);

    (10u8
        - (digits
            .iter()
            .zip(coefficient)
            .fold(0u8, |arg0: u8, (rhs, coef)| {
                u8::strict_add(arg0, *rhs * coef)
            })
            % 10))
        % 10
}

fn main() {
    let rust_in_action: Isbn = "978-3-16-148410-0".parse().unwrap();

    println!("Rust in Action's ISBN-13 ({})is valid!", rust_in_action);
}

#[test]
fn can_correctly_calculate_check_digits() {
    let cases = [
        ([9_u8, 7, 8, 1, 8, 6, 1, 9, 7, 8, 7, 6], 9_u8),
        ([9_u8, 7, 8, 3, 1, 6, 1, 4, 8, 4, 1, 0], 0_u8),
    ];

    for (case, check) in cases.iter() {
        let actual = calculate_check_digit(case);
        println!("{:?} -> {}?  {}", &case, check, actual);
        assert_eq!(calculate_check_digit(case), *check)
    }
}

#[test]
fn rust_in_action() {
    let isbn: Isbn = "978-3-16-148410-0".parse().unwrap();
    assert!(isbn.check());
}

#[test]
fn just_checking() {
    assert_eq!(0u8, 10u8 % 10);
}
