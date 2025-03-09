use fibonacci::sequence::ConsecutiveTerms;

#[test]
fn undeclared_initial_terms_fallbacks() {
    let terms: ConsecutiveTerms<u32> = ConsecutiveTerms::new(None, None).unwrap();

    assert_eq!(terms.0, 0);
    assert_eq!(terms.1, 1);
}

#[test]
fn declared_initial_terms() {
    let terms: ConsecutiveTerms<u32> = ConsecutiveTerms::new(Some(3), Some(5)).unwrap();

    assert_eq!(terms.0, 3);
    assert_eq!(terms.1, 5);
}

#[test]
fn are_first_10_consecutives_fibonacci() {
    let reference = [0, 1, 1, 2, 3, 5, 8, 13, 21, 34];
    let mut terms: ConsecutiveTerms<u32> = ConsecutiveTerms::new(None, None).unwrap();

    for i in 2..reference.len() {
        terms = terms.next().unwrap();

        assert_eq!(terms.1, reference[i]);
    }
}

#[test]
fn are_first_10_consecutives_fibonacci_backwards() {
    let reference = [34, 21, 13, 8, 5, 3, 2, 1, 1, 0];
    let mut terms: ConsecutiveTerms<u32> = ConsecutiveTerms::new(Some(21), Some(34)).unwrap();

    for i in 2..reference.len() {
        terms = terms.prev().unwrap();

        assert_eq!(terms.0, reference[i]);
    }
}