

// extern crate sha3;
// extern crate hmac;
// extern crate rand;
// extern crate num;
extern crate sha2;
extern crate digest;

use std::fmt;
use self::sha2::Sha256;
use self::digest::Digest;


pub struct Context {
    pub seed_sortition: i32,
    pub user_weights: Vec<f64>,
    pub prev_block: Block,
}

impl fmt::Display for Context {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Context: (seed_sortition: {}, user_weights: {:?}, prev_block: {:?})",
           self.seed_sortition,
           self.user_weights,
           self.prev_block,
       )
    }
}

#[derive(Debug)]
pub enum Block {
    Block
}

fn hash_eg() {
    let mut hasher = Sha256::default();
    hasher.input(b"hello world");
    let output = hasher.result();
    println!("hasher: {:?}", output);
}

pub fn BA_star(ctx: Context, round: i32, block: Block) {
    println!("\nBA* Algorithm");
}


