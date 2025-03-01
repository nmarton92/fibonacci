pub struct ConsecutiveTerms(pub u32, pub u32);

impl ConsecutiveTerms {
    fn nth_term_recurrence_relation(terms: &ConsecutiveTerms) -> u32 {
        terms.0 + terms.1
    }

    pub fn new(term_1: Option<u32>, term_2: Option<u32>) -> ConsecutiveTerms {
        let term_1 = term_1.unwrap_or(0);
        let term_2 = term_2.unwrap_or(1);

        ConsecutiveTerms(term_1, term_2)
    }

    pub fn next(&self) -> ConsecutiveTerms {
        let next = ConsecutiveTerms::nth_term_recurrence_relation(self);
        ConsecutiveTerms(self.1, next)
    }
}

#[cfg(test)]
mod tests {
    use super::ConsecutiveTerms;

    const REFERENCE_SEQUENCE: [u32; 10] = [1, 2, 3, 5, 8, 13, 21, 34, 55, 89];

    #[test]
    fn undeclared_initial_terms_fallbacks() {
        let sequence = ConsecutiveTerms::new(None, None);

        assert_eq!(sequence.0, 0);
        assert_eq!(sequence.1, 1);
    }

    #[test]
    fn declared_initial_terms() {
        let sequence = ConsecutiveTerms(3, 5);

        assert_eq!(sequence.0, 3);
        assert_eq!(sequence.1, 5);
    }

    #[test]
    fn are_first_10_consecutives_fibonacci() {
        let mut sequence: Vec<u32> = vec![0, 1];

        for i in 0..9 {
            let length = sequence.len();
            let last = sequence[length - 1];
            let before_last = sequence[length - 2];

            let terms = ConsecutiveTerms(before_last, last);

            sequence.push(terms.next().1);

            assert_eq!(sequence.last(), Some(&REFERENCE_SEQUENCE[i]));
        }
    }
}
