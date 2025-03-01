pub struct ConsecutiveTerms(pub u32, pub u32);

impl ConsecutiveTerms {
    fn nth_term_recurrence_relation(term_1: &u32, term_2: &u32) -> u32 {
        term_1 + term_2
    }

    fn nth_term_recurrence_relation_backwards(term_1: &u32, term_2: &u32) -> u32 {
        term_2 - term_1
    }

    pub fn new(term_1: Option<u32>, term_2: Option<u32>) -> Result<ConsecutiveTerms, String> {
        let term_1 = term_1.unwrap_or(0);
        let term_2 = term_2.unwrap_or(1);

        Ok(ConsecutiveTerms(term_1, term_2))
    }

    pub fn next(&self) -> Result<ConsecutiveTerms, String> {
        let next = ConsecutiveTerms::nth_term_recurrence_relation(&self.0, &self.1);
        Ok(ConsecutiveTerms(self.1, next))
    }

    pub fn prev(&self) -> Result<ConsecutiveTerms, String> {
        if self.0 == 0 {
            return Err("End of sequence.".to_string());
        }

        let previous = ConsecutiveTerms::nth_term_recurrence_relation_backwards(&self.0, &self.1);
        Ok(ConsecutiveTerms(previous, self.0))
    }
}

#[cfg(test)]
mod tests {
    use super::ConsecutiveTerms;

    #[test]
    fn undeclared_initial_terms_fallbacks() {
        let terms = ConsecutiveTerms::new(None, None).unwrap();

        assert_eq!(terms.0, 0);
        assert_eq!(terms.1, 1);
    }

    #[test]
    fn declared_initial_terms() {
        let terms = ConsecutiveTerms::new(Some(3), Some(5)).unwrap();

        assert_eq!(terms.0, 3);
        assert_eq!(terms.1, 5);
    }

    #[test]
    fn are_first_10_consecutives_fibonacci() {
        let reference = [0, 1, 1, 2, 3, 5, 8, 13, 21, 34];
        let mut terms = ConsecutiveTerms::new(None, None).unwrap();

        for i in 2..reference.len() {
            terms = terms.next().unwrap();

            assert_eq!(terms.1, reference[i]);
        }
    }

    #[test]
    fn are_first_10_consecutives_fibonacci_backwords() {
        let reference = [34, 21, 13, 8, 5, 3, 2, 1, 1, 0];
        let mut terms = ConsecutiveTerms::new(Some(21), Some(34)).unwrap();

        for i in 2..reference.len() {
            terms = terms.prev().unwrap();

            assert_eq!(terms.0, reference[i]);
        }
    }
}
