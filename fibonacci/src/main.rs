use uint::construct_uint;

construct_uint! {
    pub struct U256(4);
}

fn main() {
    let mut fibonacci: Vec<U256> = Vec::new();

    loop {
        let length: usize = fibonacci.len();

        if length == 0 {
            fibonacci.push(U256::from(0));
        } else if length == 1 {
            fibonacci.push(U256::from(&fibonacci[length - 1] + 1));
        } else {
            let last = fibonacci[fibonacci.len() - 1];
            let beforeLast = fibonacci[fibonacci.len() - 2];
            
            fibonacci.push(U256::from(last + beforeLast));
        }

        println!("{}", fibonacci[fibonacci.len() - 1]);
    }
}
