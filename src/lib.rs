//! Check if a number is prime

use std::env;
use std::ops::RangeInclusive;

/// Returns an inclusive range from command line arguments
pub fn parse_range() -> Result<RangeInclusive<u128>, &'static str> {
    let mut args = env::args();
    args.next();
    let start: u128 = match args.next() {
        Some(string) => string.parse().unwrap(),
        _ => return Err("Expected a positive whole number to test if prime"),
    };
    let end: u128 = match args.next() {
        Some(string) => string.parse().unwrap(),
        _ => start,
    };
    Ok(start..=end)
}

/// Checks if a number is prime
pub fn is_prime(number: u128) -> bool {
    match number {
        1 => false,
        2 | 3 => true,
        _ => {
            let test_divisors = Divisors::new(number);
            for divisor in test_divisors {
                if number % divisor == 0 {
                    return false
                }
            }
            true
        }
    }
}

/// Iterator of divisors to test prime numbers with
///
/// Yields 2, 3, (6n-1), (6n+1), ... sqrt(number) for a given number
struct Divisors {
    max_divisor: u128,
    minus: bool,
    n: u128,
    two_or_three: Option<u32>,
}

impl Divisors {
    fn new(number: u128) -> Divisors {
        Divisors {
            max_divisor: (number as f64).sqrt() as u128 + 1,
            minus: true,
            n: 1,
            two_or_three: Some(2),
        }
    }
}

impl Iterator for Divisors {
    type Item = u128;

    fn next(&mut self) -> Option<Self::Item> {
        // Return 2, then 3...
        if let Some(x) = self.two_or_three {
            if x == 2 {
                self.two_or_three = Some(3);
                return Some(2);
            } else {
                self.two_or_three = None;
                return Some(3);
            }
        }
        // Return (6n-1), (6n+1); n+=1...assert_eq!
        // Until max_divisor reached
        let next_term = if self.minus {
            self.minus = false;
            6 * self.n - 1
        } else {
            self.minus = true;
            self.n += 1;
            6 * (self.n - 1) + 1
        };

        if next_term <= self.max_divisor {
            Some(next_term)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn divisors_generated_correctly() {
        let expected_divisors = [2, 3, 5, 7, 11, 13, 17, 19, 23, 25, 29];
        for (expected, actual) in expected_divisors.iter().zip(Divisors::new(100)) {
            assert_eq!(expected, &actual);
        }
    }

    #[test]
    fn first_100_primes() {
        let first_100_primes = [
            2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61,
            67, 71, 73, 79, 83, 89, 97
        ];
        let mut count_primes = 0;
        for i in 1..=100 {
            if is_prime(i) {
                assert_eq!(first_100_primes[count_primes], i);
                count_primes += 1;
            }
        }
    }
}
