extern crate rand;
use rand::{thread_rng, Rng};

pub fn private_key(p: u64) -> u64 {
    let mut rng = thread_rng();
    rng.gen_range(2, p)
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    let powered = g.pow(a as u32);
    modulus(powered, p)
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    let powered = b_pub.pow(a as u32);
    modulus(powered, p)
}

// Because Rust doesn't seem to have a mod function
fn modulus(n: u64, m: u64) -> u64 {
    ((n % m) + m) % m
}