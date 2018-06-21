
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unused_imports)]


mod _1_key_exchange;
mod _2_parse_date;
mod _3_enums_traits;
mod _4_listc;
mod _5_algorand;

extern crate chrono;
#[macro_use] extern crate proptest;
// Macro must preceded extern crate proptest

use std::vec::Vec;
use _3_enums_traits::Language;
use _5_algorand::{ Context, Block };


fn main() {
    // // _1_key_exchange
    // _1_key_exchange::key_exchange();
    // // _2_parse_date
    // _2_parse_date::print_parsed_date("17th of June 2018");
    // // _3_greet Traits
    // // let greeter = _3_enums_traits::Greeter::new().with_language(Language::German);
    // // _4_listc::listc();

    let mut genesis_block = Block{
        tx: vec![0.0]
    };
    let mut ctx = Context{
        seed_sortition: 1,
        user_weights: vec![0.4,0.4,0.1,0.1],
        prev_block: genesis_block,
    };
    let mut round: i32 = 1;
    let mut block = Block{ tx: vec![1.0,2.0,3.0] };
    _5_algorand::BA_star(ctx, round, block);
}
