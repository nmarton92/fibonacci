use std::env;
use fibonacci::{sequence, formula};

enum Operation {
    Sequence(u32),
    Binet(u32),
}

fn on_sequence(start_from_nth: u32) {
    let iterator = sequence::ConsecutiveTerms::new(start_from_nth);

    for i in iterator {
        println!("{}", i);
    }
}

fn on_binet(nth: u32) {
    println!("{}", formula::binet_formula(nth));
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let args1 = match args.get(1) {
        Some(value) => value,
        None => panic!("No given operation."),
    };

    let args2 = match args.get(2) {
        Some(value) => value.parse().unwrap_or_default(),
        None => panic!("Invalid parameter."),
    };

    let operation = match args1.as_str() {
        "-s" | "--sequence" => Operation::Sequence(args2),
        "-b" | "--binet" => Operation::Binet(args2),
        _ => panic!("Unknown operation."),
    };

    match operation {
        Operation::Sequence(value) => on_sequence(value),
        Operation::Binet(value) => on_binet(value),
    }
}
