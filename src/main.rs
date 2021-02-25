use std::env;
use std::process;

fn main() {
    let range = primes::parse_range(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    for i in range {
        if primes::is_prime(i) {
            println!("{}", i);
        }
    }
}
