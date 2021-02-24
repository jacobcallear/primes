pub fn is_prime(number: u64) -> bool {
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

struct Divisors {
    max_divisor: u64,
    minus: bool,
    n: u64,
    divisor: u32,
}

impl Divisors {
    fn new(number: u64) -> Divisors {
        Divisors {
            max_divisor: (number as f64).sqrt() as u64 + 1,
            minus: true,
            n: 1,
            divisor: 2,
        }
    }
}

impl Iterator for Divisors {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.divisor == 2 {
            self.divisor = 3;
            return Some(2);
        } else if self.divisor == 3 {
            self.divisor = 4;
            return Some(3);
        };
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
