
#![allow(dead_code)]

mod _1_key_exchange;
mod _2_parse_date;

extern crate chrono;
extern crate proptest;

use chrono::NaiveDate;


fn main() {
    _1_key_exchange::key_exchange();

    let date = _2_parse_date::parse_date("17th of June 2018");
    match date {
        Ok(d) => println!(
            "\nThe date input is: {}\nWhich is: {:?}",
            d.format("%A, %-d %B, %C%y").to_string(),
            d.format("%Y-%m-%d").to_string(),
            ),
        Err(e) => println!("\nInvalid date!"),
    }
}
