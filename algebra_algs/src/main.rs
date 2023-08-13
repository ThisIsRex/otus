use std::time::Instant;

use pow::MyPow;

use crate::{fibonacci::find_rec, prime::find_prime_count};

mod fibonacci;
mod pow;
mod prime;

fn main() {
    println!("{}", 2f64.pow_iter(10));
    println!("{}", 2f64.pow_rec(10));
    println!("{}", 2f64.pow_sq_rec(10));
    println!("{}", 2f64.pow_sq_iter(10));

    println!("{}", fibonacci::find_iter(100));
    println!("{}", fibonacci::find_golden(100));

    println!("{}", prime::find_eratosthenes_linear(1000).len());
}
