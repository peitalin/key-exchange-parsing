
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unused_imports)]


mod _1_key_exchange;
mod _2_parse_date;
mod _3_enums_traits;
mod _4_listc;
mod _5_algorand;
mod _6_network;

extern crate chrono;
#[macro_use] extern crate proptest;
#[macro_use(c)] extern crate cute;
// Macro must preceded extern crate proptest

use std::vec::Vec;
use _3_enums_traits::Language;
use _5_algorand::{ Context, Block, hash_block };


fn main() {
    // // _1_key_exchange
    // _1_key_exchange::key_exchange();
    // // _2_parse_date
    // _2_parse_date::print_parsed_date("17th of June 2018");
    // // _3_greet Traits
    // // let greeter = _3_enums_traits::Greeter::new().with_language(Language::German);
    // // _4_listc::listc();

    // Define Genesis Block
    let mut genesis_block = Block{
        prev_hash: 0,
        tx: vec![1, 20, 300, 400]
    };
    // Define Contect,
    let mut ctx = Context{
        seed_sortition: 1,
        user_weights: vec![0.4,0.4,0.1,0.1],
        prev_block: &genesis_block, // Reference to genesis bloc
    };
    let mut round: i32 = 1;
    _5_algorand::BA_star(&ctx, round, &genesis_block);
    _6_network::serve();
}
