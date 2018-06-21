

// extern crate sha3;
// extern crate hmac;
// extern crate rand;
// extern crate num;
extern crate sha2;
extern crate digest;

use std::fmt;
use self::sha2::Sha256;
use self::digest::Digest;
use std::hash::{ Hash, Hasher };


// Defined: Micali et al., (2018, page 8)
pub struct Context {
    pub seed_sortition: i32,
    pub user_weights: Vec<f64>,
    pub prev_block: Block,
}

impl fmt::Display for Context {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(seed_sortition: {}, user_weights: {:?}, prev_block: {:?})",
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
    println!("Params:");
    println!("\tContext: {}", ctx);
    println!("\tRound: {}", round);
    println!("\tBlock: {:?}", block);

    let mut hblock = Reduction(&ctx, round, &block);
    let mut hblock_star = BinaryBA_star(&ctx, round, &hblock);

    // Count Votes
    let T_final = 0.5;
    let tau_final = 0.5;
    let lambda_step = 0.01;
    let mut r = CountVotes(&ctx, round, ConsensusType::FINAL, T_final, tau_final, lambda_step);

    // Hash Block
    let block_of_hashes = BlockOfHash(hblock_star)

    if hblock_star == r {
        return  ConsensusType::FINAL
    } else {
        return ConsensusType::TENTATIVE
    }
}

#[derive(Debug)]
pub enum ConsensusType {
    FINAL,
    TENTATIVE,
}

fn BlockOfHash(hblock_star: Block) {
    unimplemented!();
}

fn Reduction(ctx: &Context, round: i32, block: &Block) -> Block::Block {
    println!("\nReduction():");
    println!("\tWas passed Context: {}", ctx);
    // unimplemented!();
}

fn BinaryBA_star(ctx: &Context, round: i32, block: &Block) {
    println!("\nBinaryBA*():");
    println!("\tWas passed Context: {}", ctx);
    // unimplemented!();
}

fn CountVotes(ctx: &Context, round: i32, Final: ConsensusType, T_final: f64, tau_final: f64, lambda_step: f64) {
    unimplemented!();
}


