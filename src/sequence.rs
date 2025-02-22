pub struct ConsecutiveTerms(pub u32, pub u32);

impl ConsecutiveTerms {
    fn nth_term_recurrence_relation(terms: &ConsecutiveTerms) -> u32 {
        terms.0 + terms.1
    }

    pub fn next(&self) -> ConsecutiveTerms {
        let next = ConsecutiveTerms::nth_term_recurrence_relation(self);
        ConsecutiveTerms(self.1, next)
    }
}
