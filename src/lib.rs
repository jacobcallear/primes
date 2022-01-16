//! Check if a number is prime

use std::env;
use std::ops::RangeInclusive;

/// Returns an inclusive range from command line arguments
pub fn parse_range(
    mut args: env::Args,
) -> Result<RangeInclusive<u128>, &'static str> {
    args.next();

    let start: u128 = match args.next() {
        Some(string) => match string.parse() {
            Ok(num) => num,
            _ => return Err("expected a positive whole number"),
        },
        _ => return Err("expected at least one positive whole number"),
    };

    let end = match args.next() {
        Some(string) => match string.parse() {
            Ok(num) => num,
            _ => return Err("expected a positive whole number"),
        },
        _ => start,
    };

    Ok(start..=end)
}

/// Checks if a number is prime
///
/// ```
/// use primes::is_prime;
///
/// assert_eq!(is_prime(2), true);
/// assert_eq!(is_prime(100), false);
/// assert_eq!(is_prime(483957312131), true);
/// ```
pub fn is_prime(number: u128) -> bool {
    match number {
        0 | 1 => false,
        2 => true,
        _ => Divisors::new(number).all(|divisor| number % divisor != 0),
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
            max_divisor: (number as f64).sqrt() as u128,
            minus: true,
            n: 1,
            two_or_three: Some(2),
        }
    }
}

impl Iterator for Divisors {
    type Item = u128;

    fn next(&mut self) -> Option<Self::Item> {
        let next_term = match self.two_or_three {
            // Return 2, then 3...
            Some(2) => {
                self.two_or_three = Some(3);
                2
            }
            Some(_) => {
                self.two_or_three = None;
                3
            }
            // ...then (6n-1, 6n+1; n+=1)
            None => {
                if self.minus {
                    self.minus = false;
                    6 * self.n - 1
                } else {
                    self.minus = true;
                    self.n += 1;
                    6 * (self.n - 1) + 1
                }
            }
        };

        // Stop iteration when max_divisor reached
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
        let expected_divisors_for_100: Vec<u128> = vec![2, 3, 5, 7];
        let actual_divisors_for_100: Vec<u128> = Divisors::new(100).collect();

        assert_eq!(expected_divisors_for_100, actual_divisors_for_100);
    }

    #[test]
    fn first_100_primes() {
        let expected_primes_below_100: Vec<u128> = vec![
            2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61,
            67, 71, 73, 79, 83, 89, 97,
        ];
        let actual_primes_below_100: Vec<u128> =
            (1..100).filter(|number| is_prime(*number)).collect();

        assert_eq!(expected_primes_below_100, actual_primes_below_100);
    }
}
