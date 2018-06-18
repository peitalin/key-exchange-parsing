


use chrono;
use chrono::NaiveDate;
use proptest;
use std;


#[derive(Debug, PartialEq)]
pub enum Error {
    ParseError,
    InvalidDateError,
    InvalidDayError,
}

impl From<std::num::ParseIntError> for Error {
    fn from(t: std::num::ParseIntError) -> Error {
        Error::ParseError
    }
}


pub fn parse_month(month: &str) -> Option<usize> {
    let months = vec![
        "January"
        , "February"
        , "March"
        , "April"
        , "May"
        , "June"
        , "July"
        , "August"
        , "September"
        , "October"
        , "November"
        , "December"
    ];
    match months.iter().position(|&elem| elem == month) {
        Some(index) => Some(index + 1),
        None => None,
    }
}

pub fn parse_day(day_with_ordinal: &str) -> Result<u32, Error> {
    let day: u32 = day_with_ordinal
        .chars()
        .take_while(|c| c.is_digit(10))
        .collect::<String>()
        .parse::<u32>()?;

    let ordinal = day_with_ordinal
        .chars()
        .skip_while(|c| c.is_digit(10))
        .collect::<String>();

    match (day, ordinal.as_ref()) {
        (1, "st")
        | (2, "nd")
        | (3, "rd")
        | (4...20, "th")
        | (21, "st")
        | (22, "nd")
        | (23, "rd")
        | (24...30, "th")
        | (31, "st") => Ok(day),
        _ => Err(Error::InvalidDayError)
    }
}

pub fn parse_date(humandate: impl AsRef<str>) -> Result<NaiveDate, Error> {
    let parts: Vec<&str> = humandate.as_ref().split_whitespace().collect();
    if parts.len() != 4 {
        return Err(Error::ParseError);
    }
    let day: u32 = parse_day(parts[0])?;
    // TO DO: parts[1] == of;
    let month: u32 = parse_month(parts[2]).ok_or_else(|| Error::ParseError)? as u32;
    let year: i32 = parts[3].parse()?;
    NaiveDate::from_ymd_opt(year, month, day).ok_or_else(|| Error::InvalidDateError)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            parse_date("4th of September 2015"),
            Ok(NaiveDate::from_ymd(2015, 9, 4))
        );
        assert_eq!(
            parse_date("21st of December 2015"),
            Ok(NaiveDate::from_ymd(2015, 12, 21))
        );
    }

    proptest! {
        #[test]
        fn doesnt_crash(ref s in "\\PC*") {
            parse_date(s);
        }
    }

}
