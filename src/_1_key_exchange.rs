#![allow(dead_code)]

// extern crate sha2;
// extern crate sha3;
// extern crate hmac;
// extern crate rand;
extern crate num;
use self::num::pow;

pub fn key_exchange() {
    // Must agree on a shared prime and shared base
    let shared_prime: usize = 23;
    // p
    let shared_base: usize = 5;
    // g

    // Alice and Bob then have their own secret keys
    let alice_secret: usize = 6;
    // a
    let bob_secret: usize = 7;
    // b

    // Begin
    println!("Publicly shared variables");
    println!("\tPublicly shared prime: {}", shared_prime);
    println!("\tPublicly shared base: {}", shared_base);

    // Alice sends Bob A = g^a mod p
    let A: usize = (pow(shared_base, alice_secret)) % shared_prime;
    println!("Alice sends message over public channel: {} (A = g^a mod p)", A);

    // Bob sends to Alice B = g^b mod p
    let B: usize = (pow(shared_base, bob_secret)) % shared_prime;
    println!("Bob sends message over public channel: {} (B = g^b mod p)\n", B);

    // Alice calculates secret
    let alice_shared_secret: usize = (pow(B, alice_secret)) % shared_prime;
    println!("Alice privately calculates secret:\n\ts = A^b mod p = {}", alice_shared_secret);

    // Bob calculates secret
    let bob_shared_secret: usize = (pow(A, bob_secret)) % shared_prime;
    println!("Bob privately calculates secret:\n\ts = B^a mod p = {}", bob_shared_secret);

    println!("Alice and Bob can use the shared secret key to communicate over insecure channels")
}
