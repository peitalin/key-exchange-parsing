

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
use std::collections::hash_map::DefaultHasher;


//////////////// TYPES //////////////
// Defined: Micali et al., (2018, page 8)
pub struct Context<'a> {
    pub seed_sortition: i32,
    pub user_weights: Vec<f64>,
    pub prev_block: &'a Block,
}
impl<'a> fmt::Display for Context<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\n\t\t(seed_sortition: {}, \n\t\tuser_weights: {:?}, \n\t\tprev_block: {:?})",
           self.seed_sortition,
           self.user_weights,
           self.prev_block,
       )
    }
}

#[derive(Debug)]
pub struct Block {
    pub prev_hash: u64,
    pub tx: Vec<u64>
}
impl Hash for Block {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let mut hasher = DefaultHasher::new();
        Hash::hash_slice(&self.tx, &mut hasher);
        self.tx.hash(state);
    }
}

#[derive(Debug)]
pub enum ConsensusType {
    FINAL,
    TENTATIVE,
}




//////////////// FUNCTIONS //////////////
pub fn hash_block(block: &Block) -> u64 {
    let mut s = DefaultHasher::new();
    block.hash(&mut s);
    s.finish()
}

pub fn BA_star(ctx: &Context, round: i32, block: &Block) -> ConsensusType {
    println!("\nBA* Algorithm");
    println!("Params:");
    println!("\tContext: {}", ctx);
    println!("\tRound: {}", round);
    println!("\tBlock: {:?}", block);

    let mut hblock      = Reduction(&ctx, round, &block);
    let mut hblock_star = BinaryBA_star(&ctx, round, &hblock);

    // Count Votes
    let T_final     = 0.5;
    let tau_final   = 0.3;
    let lambda_step = 0.01;
    // Check if we reach FINAL or TENTATIVE consensus
    let r = CountVotes(&ctx, round, ConsensusType::FINAL, T_final, tau_final, lambda_step);
    // Hash Block
    let block_of_hashes = BlockOfHash(hblock_star);

    // if hblock_star == r {
    //     return  ConsensusType::FINAL
    // } else {
    //     return ConsensusType::TENTATIVE
    // }
    return ConsensusType::FINAL
}

fn BlockOfHash<'a>(hblock_star: &'a Block) -> &'a Block {
    println!("\nBlockOfHash():");
    println!("\thblock_star: {:?}", hblock_star);
    hblock_star
}

fn Reduction<'a>(ctx: &Context, round: i32, block: &'a Block) -> Block {
    // Converts the problem of reaching consensus on an arbitrary value (hash of block)
    // to reaching consensus on one of two values:
    // either a specific proposed block hash, of the hash of an empty block
    // Micali et al. (page 9)

    println!("\nReduction():");
    println!("\tInput Context: {}", ctx);
    Block{
        prev_hash: hash_block(&block),
        tx: vec![1, 20, 300, 400].iter().map(|&n| n+1).collect(),
    }
}

fn BinaryBA_star<'a>(ctx: &Context, round: i32, block: &'a Block) -> &'a Block {
    // Executes until there is consensus on either block_hash, or empty_hash
    println!("\nBinaryBA*():");
    println!("\tInput Context: {}", ctx);
    return block
    // unimplemented!();
}

fn CountVotes(ctx: &Context, round: i32, Final: ConsensusType, T_final: f64, tau_final: f64, lambda_step: f64) {
    // unimplemented!();
}


fn CommitteeVote() {
    // Sortition to check if user is in committee
    // If so, start Gossip protocol to relay blocks/proofs
    //
}





