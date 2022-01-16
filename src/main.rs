use primes::{is_prime, parse_range};
use std::env;
use std::process;

fn main() {
    let range = parse_range(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    for i in range {
        if is_prime(i) {
            println!("{}", i);
        }
    }
}
