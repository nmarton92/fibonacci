fn main() {
    let mut fibonacci: Vec<u128> = Vec::new();

    loop {
        let length: usize = fibonacci.len();

        if length == 0 {
            fibonacci.push(0);
        } else if length == 1 {
            fibonacci.push(&fibonacci[length - 1] + 1);
        } else {
            let last = fibonacci[fibonacci.len() - 1];
            let beforeLast = fibonacci[fibonacci.len() - 2];
            
            fibonacci.push(last + beforeLast);
        }

        println!("{}", fibonacci[fibonacci.len() - 1]);
    }
}
