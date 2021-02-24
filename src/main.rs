use primes;

fn main() {
    for i in 1..100 {
        if primes::is_prime(i) {
            println!("{} is prime", i);
        }
    }
}
