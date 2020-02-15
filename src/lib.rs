#[derive(Debug)]
pub struct Sieve {
    max: usize,
    sieve_table: Vec<bool>,
    filled: bool,
}

impl Sieve {
    /// Create a new prime sieve with the maximum value `max`, but *do not* populate it.
    /// ```
    /// let unfilled_sieve = prime_sieve::Sieve::unfilled(10);
    ///
    /// // Returns Err()
    /// unfilled_sieve.lookup(5);
    /// ```
    pub fn unfilled(max: usize) -> Sieve {
        Sieve {
            max,
            sieve_table: (0..=max).map(|_| true).collect(),
            filled: false,
        }
    }

    /// Create and populate a prime sieve with the maximum value `max`.
    /// ```
    /// let my_sieve = prime_sieve::Sieve::new(10);
    /// ```
    pub fn new(max: usize) -> Sieve {
        let mut result = Sieve::unfilled(max);
        result.fill();
        result
    }

    /// Get the max value of this sieve
    ///
    /// ```
    /// let my_sieve = prime_sieve::Sieve::new(10);
    /// assert_eq!(my_sieve.max(), 10);
    /// ```
    pub fn max(self) -> usize {
        return self.max;
    }

    // Warning: doesn't check if the target is out of bounds
    fn process_ahead(&mut self, target: usize) {
        if !self.sieve_table[target] {
            return;
        }
        let mut cur_target: usize = 2 * target;
        while cur_target <= self.max {
            self.sieve_table[cur_target] = false;
            cur_target += target;
        }
    }

    /// Populate an unfilled sieve - note that the sieve must be `mut`.
    ///
    /// Has no effect on already-filled sieves.
    ///
    /// ```
    /// let mut my_sieve = prime_sieve::Sieve::unfilled(100);
    ///
    /// // Returns Err()
    /// my_sieve.lookup(10);
    ///
    /// my_sieve.fill();
    ///
    /// assert_eq!(my_sieve.lookup(10).unwrap(), false);
    /// ```
    pub fn fill(&mut self) {
        if self.filled {
            return;
        }
        self.sieve_table[0] = false;
        self.sieve_table[1] = false;
        for i in 2..=((self.max as f64).sqrt() as usize) {
            self.process_ahead(i);
        }
        self.filled = true;
    }

    /// Determine whether a number within the prime sieve's limits is trule prime or not
    ///
    /// Returns `Err()` if sieve is unpopulated or if `target > sieve.max()`.
    pub fn lookup(&self, target: usize) -> Result<bool, String> {
        if !self.filled {
            Err(String::from("Sieve not populated!"))
        } else if target > self.max {
            Err(format!(
                "{} is out of this sieve's bounds (max {})",
                target, self.max
            ))
        } else {
            Ok(self.sieve_table[target])
        }
    }

    /// Takes a vector of `usize`s and removes all the non-prime ones.
    ///
    /// Will return `Err()` if one of `target`'s elements is outside the bounds of this sieve
    ///
    /// ```
    /// let my_sieve = prime_sieve::Sieve::new(100);
    ///
    /// let filtered = my_sieve.filter(vec![1,2,3,4]).unwrap();
    /// assert_eq!(filtered, vec![2,3]);
    /// ```
    pub fn filter(&self, target: Vec<usize>) -> Result<Vec<usize>, String> {
        let mut result: Vec<usize> = Vec::new();
        for i in target.into_iter() {
            if self.lookup(i)? {
                result.push(i);
            }
        }
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::assert;
    // use test::Bencher;

    #[test]
    fn marks_correctly_low() {
        let test_case = Sieve::new(10);
        assert!(test_case.lookup(5).unwrap());
    }

    #[test]
    fn marks_correctly_med() {
        let test_case = Sieve::new(1000);
        assert!(!test_case.lookup(500).unwrap());
    }

    #[test]
    fn test_correct_filtering() {
        let test_sieve = Sieve::new(15);
        let cases = vec![1, 2, 3, 4, 5, 6];
        assert_eq!(vec![2, 3, 5], test_sieve.filter(cases).unwrap());
    }

    #[test]
    #[should_panic]
    fn panics_on_invalid_lookup() {
        let test_case = Sieve::new(10);
        test_case.lookup(100).unwrap();
    }

    #[test]
    #[should_panic]
    fn panics_on_unfilled_lookup() {
        let test_case = Sieve::unfilled(10);
        test_case.lookup(5).unwrap();
    }

    // #[bench]
    // fn ten_million(b: &mut Bencher) {
    //     let mut test_case = Sieve::new(10_000_000);
    //     b.iter(|| test_case.fill());
    // }
}
