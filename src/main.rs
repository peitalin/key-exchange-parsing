
#![allow(dead_code)]
#![allow(non_snake_case)]

mod _1_key_exchange;
mod _2_parse_date;

extern crate chrono;
#[macro_use] extern crate proptest;
// Macro must preceded extern crate proptest


fn main() {
    // _1_key_exchange
    _1_key_exchange::key_exchange();

    // _2_parse_date
    match _2_parse_date::parse_date("17th of June 2018") {
        Ok(d) => println!(
                "\nThe date input is: {}\nWhich is: {}",
                d.format("%A, %-d %B, %C%y").to_string(),
                d.format("%Y-%m-%d").to_string()
            ),
        Err(_e) => println!("\nInvalid date!"),
    }
}
