pub fn binet_formula(nth: u32) -> u32 {
    let sqrt_of_5 = 5f64.sqrt();
    let phi = (1.0 + sqrt_of_5) / 2.0;
    let psi = (1.0 - sqrt_of_5) / 2.0;

    ((phi.powi(nth as i32) - psi.powi(nth as i32)) * (1.0 / sqrt_of_5)) as u32
}

mod tests {
    #[test]
    fn get_the_first_ten_fibonacci_with_binet_formula() {
        let reference = [0, 1, 1, 2, 3, 5, 8, 13, 21, 34];

        for i in 0..reference.len() {
            assert_eq!(super::binet_formula(i as u32), reference[i]);
        }
    }
}
