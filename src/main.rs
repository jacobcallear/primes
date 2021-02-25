use primes;

fn main() {
    let range = match primes::parse_range() {
        Ok(range) => range,
        Err(message) => panic!(message),
    };

    for i in range {
        if primes::is_prime(i) {
            println!("{}", i);
        }
    }
}
