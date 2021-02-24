fn main() {
    println!("2: {}", is_prime(2));
    println!("100: {}", is_prime(100));
    println!("1000000000000: {}", is_prime(100000000000000000));
}

fn is_prime(number: u64) -> bool {
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
}

impl Divisors {
    fn new(number: u64) -> Divisors {
        Divisors {
            max_divisor: (number as f64).sqrt() as u64 + 1,
            minus: true,
            n: 1,
        }
    }
}

impl Iterator for Divisors {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.minus {
            self.minus = false;
            let next_term = 6 * self.n - 1;
            if next_term <= self.max_divisor {
                Some(next_term)
            } else {
                None
            }
        } else {
            self.minus = true;
            let next_term = 6 * self.n + 1;
            self.n += 1;
            if next_term <= self.max_divisor {
                Some(next_term)
            } else {
                None
            }
        }
    }
}
