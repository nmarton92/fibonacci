use crate::formula;

pub struct ConsecutiveTerms(u32, u32);

impl ConsecutiveTerms {
    /// Creates two consecutives from the Fibonacci sequence.
    /// It calculates the number at the given `nth` position, and the preceeding number.
    /// 
    /// # Example
    /// ```
    /// use fibonacci::sequence::ConsecutiveTerms;
    /// let mut fibonacci = ConsecutiveTerms::new(6);
    /// ```
    pub fn new(nth: u32) -> ConsecutiveTerms {
        let current = formula::binet_formula(nth);
        let previous = if nth > 1 { formula::binet_formula(nth - 1) } else { 0 };

        ConsecutiveTerms(previous, current)
    }

    fn recurrence_relation(previous: &u32, current: &u32) -> u32 {
        if *previous == 0 {
            1
        } else {
            previous + current
        }
    }
}

impl Iterator for ConsecutiveTerms {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let next = ConsecutiveTerms::recurrence_relation(&self.0, &self.1);

        self.0 = self.1;
        self.1 = next;

        Some(self.1)
    }
}

#[cfg(test)]
mod tests {
    use super::ConsecutiveTerms;

    #[test]
    fn starts_with_zero() {
        let mut fibonacci = ConsecutiveTerms::new(0);

        assert_eq!(fibonacci.0, 0);

        fibonacci.next();

        assert_eq!(fibonacci.1, 1);
        assert_eq!(fibonacci.0, 0);
    }

    #[test]
    fn starts_with_one() {
        let mut fibonacci = ConsecutiveTerms::new(1);

        assert_eq!(fibonacci.0, 0);

        fibonacci.next();

        assert_eq!(fibonacci.1, 1);
        assert_eq!(fibonacci.0, 1);
    }

    #[test]
    fn starts_with_greater_than_one() {
        let mut fibonacci = ConsecutiveTerms::new(5);

        assert_eq!(fibonacci.0, 3);

        fibonacci.next();

        assert_eq!(fibonacci.0, 5);
        assert_eq!(fibonacci.1, 8);
    }
}
