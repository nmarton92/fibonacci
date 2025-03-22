use crate::formula;

pub struct Sequence {
    previous: u32,
    current: u32,
}

impl Sequence {
    pub fn new(nth: u32) -> Sequence {
        let current = formula::binet_formula(nth);
        let previous = if nth > 1 { formula::binet_formula(nth - 1) } else { 0 };

        Sequence {
            previous,
            current,
        }
    }

    pub fn recurrence_relation(previous: &u32, current: &u32) -> u32 {
        if *previous == 0 {
            1
        } else {
            previous + current
        }
    }
}

impl Iterator for Sequence {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let next = Sequence::recurrence_relation(&self.previous, &self.current);

        self.previous = self.current;
        self.current = next;

        Some(self.current)
    }
}

#[cfg(test)]
mod tests {
    use super::Sequence;

    #[test]
    fn sequence_starts_with_zero() {
        let mut fibonacci = Sequence::new(0);

        fibonacci.next();

        assert_eq!(fibonacci.current, 1);
        assert_eq!(fibonacci.previous, 0);

        fibonacci.next();

        assert_eq!(fibonacci.current, 1);
        assert_eq!(fibonacci.previous, 1);
    }

    #[test]
    fn check_five_consecutive_terms() {
        let mut fibonacci = Sequence::new(3);

        assert_eq!(fibonacci.next(), Some(3));
        assert_eq!(fibonacci.next(), Some(5));
        assert_eq!(fibonacci.next(), Some(8));
        assert_eq!(fibonacci.next(), Some(13));
        assert_eq!(fibonacci.next(), Some(21));
    }
}
