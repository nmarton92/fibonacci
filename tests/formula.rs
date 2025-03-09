use fibonacci::formula;

#[test]
fn get_the_first_ten_fibonacci_with_binet_formula() {
    let reference = [0, 1, 1, 2, 3, 5, 8, 13, 21, 34];

    for i in 0..reference.len() {
        assert_eq!(formula::binet_formula(i as u32), reference[i]);
    }
}
