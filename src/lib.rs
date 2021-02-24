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
            6 * self.n + 1
        };

        if next_term <= self.max_divisor {
            Some(next_term)
        } else {
            None
        }
    }
}
