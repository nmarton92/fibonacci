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

    fn recurrence_relation(previous: &u32, current: &u32) -> u32 {
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
    fn starts_with_zero() {
        let mut fibonacci = Sequence::new(0);

        assert_eq!(fibonacci.previous, 0);

        fibonacci.next();

        assert_eq!(fibonacci.current, 1);
        assert_eq!(fibonacci.previous, 0);
    }

    #[test]
    fn starts_with_one() {
        let mut fibonacci = Sequence::new(1);

        assert_eq!(fibonacci.previous, 0);

        fibonacci.next();

        assert_eq!(fibonacci.current, 1);
        assert_eq!(fibonacci.previous, 1);
    }

    #[test]
    fn starts_with_greater_than_one() {
        let mut fibonacci = Sequence::new(5);

        assert_eq!(fibonacci.previous, 3);

        fibonacci.next();

        assert_eq!(fibonacci.previous, 5);
        assert_eq!(fibonacci.current, 8);
    }
}
