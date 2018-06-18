
#![allow(dead_code)]

mod key_exchange;
mod parse_date;

extern crate chrono;
extern crate proptest;


fn main() {
    key_exchange::key_exchange();
    // parse_date::parse_date("17th June 2018");
}
