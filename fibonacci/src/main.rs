use uint::construct_uint;

construct_uint! {
    pub struct U256(4);
}

fn main() {
    let mut fibonacci: Vec<U256> = vec![];

    fibonacci.push(U256::from(0));
    fibonacci.push(U256::from(1));

    loop {
        let length: usize = fibonacci.len();

        let last = fibonacci[length - 1];
        let before_last = fibonacci[length - 2];

        let next_fibonacci = U256::from(last + before_last);
        
        fibonacci.push(next_fibonacci);

        println!("{}", next_fibonacci);
    }
}
