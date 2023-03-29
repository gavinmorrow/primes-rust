mod primes {
    /// Gets the prime factors of a number.
    pub fn prime_factor(n: i32) -> Vec<i32> {
        let mut factors = if n < 0 { vec![-1] } else { Vec::new() };
        let mut curr = n.abs() as u32;
        while curr != 1 {
            for prime in primes() {
                if curr % prime == 0 {
                    factors.push(prime as i32);
                    curr /= prime;
                    break;
                }
            }
        }
        factors
    }

	/// An iterator that generates prime numbers.
    pub struct Primes {
        prev: Vec<u32>,
        curr: u32,
    }

    impl Iterator for Primes {
        type Item = u32;

        fn next(&mut self) -> Option<u32> {
            let mut curr = self.curr + 1;
            'main_loop: loop {
                for prime in &self.prev {
                    if curr % prime == 0 {
                        curr += 1;
                        continue 'main_loop;
                    }
                }
                break;
            }
            self.prev.push(curr);
            Some(curr)
        }
    }

	/// Returns an iterator that generates prime numbers.
    pub fn primes() -> Primes {
        Primes {
            prev: Vec::new(),
            curr: 1,
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_first_100_primes() {
            let primes = primes().take(100).collect::<Vec<u32>>();
            #[rustfmt::skip]
			assert_eq!(primes, vec![
				2,   3,   5,   7,   11,  13,  17,  19,  23,  29, 
				31,  37,  41,  43,  47,  53,  59,  61,  67,  71, 
				73,  79,  83,  89,  97,  101, 103, 107, 109, 113, 
				127, 131, 137, 139, 149, 151, 157, 163, 167, 173, 
				179, 181, 191, 193, 197, 199, 211, 223, 227, 229, 
				233, 239, 241, 251, 257, 263, 269, 271, 277, 281, 
				283, 293, 307, 311, 313, 317, 331, 337, 347, 349, 
				353, 359, 367, 373, 379, 383, 389, 397, 401, 409, 
				419, 421, 431, 433, 439, 443, 449, 457, 461, 463, 
				467, 479, 487, 491, 499, 503, 509, 521, 523, 541
			]);
        }

        fn test_prime_factor(n: i32, factors: Vec<i32>) {
            assert_eq!(prime_factor(n), factors);
        }

        #[test]
        fn test_prime_factor_1() {
            test_prime_factor(1, vec![]);
        }

        #[test]
        fn test_prime_factor_2() {
            test_prime_factor(2, vec![2]);
        }

        #[test]
        fn test_prime_factor_25() {
            test_prime_factor(25, vec![5, 5]);
        }

        #[test]
        fn test_prime_factor_100() {
            test_prime_factor(100, vec![2, 2, 5, 5]);
        }

        #[test]
        fn test_prime_factor_60() {
            test_prime_factor(60, vec![2, 2, 3, 5]);
        }

        #[test]
        fn test_prime_factor_9() {
            test_prime_factor(9, vec![3, 3]);
        }
    }
}
