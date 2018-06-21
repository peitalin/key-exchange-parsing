
#![allow(dead_code)]
#![allow(non_snake_case)]

mod _1_key_exchange;
mod _2_parse_date;
mod _3_enums_traits;
mod _4_listc;

extern crate chrono;
#[macro_use] extern crate proptest;
// Macro must preceded extern crate proptest

use _3_enums_traits::Language;


fn main() {
    // _1_key_exchange
    _1_key_exchange::key_exchange();
    // _2_parse_date
    _2_parse_date::print_parsed_date("17th of June 2018");
    let greeter = _3_enums_traits::Greeter::new().with_language(Language::German);

    _4_listc::listc();
}
