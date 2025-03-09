use num_traits::Num;
use std::ops::Add;

pub struct ConsecutiveTerms<T: Num>(pub T, pub T);

impl<T> ConsecutiveTerms<T>
where
    T: Num + Add<Output = T> + Clone + Copy,
{
    fn nth_term_recurrence_relation(term_1: &T, term_2: &T) -> T {
        term_1.clone() + term_2.clone()
    }

    fn nth_term_recurrence_relation_backwards(term_1: &T, term_2: &T) -> T {
        term_2.clone() - term_1.clone()
    }

    pub fn new(term_1: Option<T>, term_2: Option<T>) -> Result<ConsecutiveTerms<T>, String> {
        let term_1 = term_1.unwrap_or(T::zero());
        let term_2 = term_2.unwrap_or(T::one());

        Ok(ConsecutiveTerms(term_1, term_2))
    }

    pub fn next(&self) -> Result<ConsecutiveTerms<T>, String> {
        let next = ConsecutiveTerms::nth_term_recurrence_relation(&self.0, &self.1);
        Ok(ConsecutiveTerms(self.1, next))
    }

    pub fn prev(&self) -> Result<ConsecutiveTerms<T>, String> {
        if self.0 == T::zero() {
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
    fn recurrence_relation() {
        let term = ConsecutiveTerms::nth_term_recurrence_relation(&3, &5);

        assert_eq!(term, 8)
    }

    #[test]
    fn recurrence_relation_backwards() {
        let term = ConsecutiveTerms::nth_term_recurrence_relation_backwards(&3, &5);

        assert_eq!(term, 2)
    }
}
