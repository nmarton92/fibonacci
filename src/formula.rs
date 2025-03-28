/// Calculates the number in the sequence at the `nth` position.
/// 
/// # Example
/// 
/// ```
/// use fibonacci::formula::binet_formula;
/// let fifthElement = binet_formula(5); // results: 3
/// ```
pub fn binet_formula(nth: u32) -> u32 {
    let sqrt_of_5 = 5f64.sqrt();
    let phi = (1.0 + sqrt_of_5) / 2.0;
    let psi = (1.0 - sqrt_of_5) / 2.0;

    ((phi.powi(nth as i32) - psi.powi(nth as i32)) * (1.0 / sqrt_of_5)) as u32
}
