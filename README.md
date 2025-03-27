# Fibonacci generator

A small library that generates the Fibonacci sequence and provides a utility to retrieve the nth term.

## Console interface

### `--sequence, -s [start_from_nth]`

Prints the Fibonacci sequence starting from `start_from_nth`.

#### Parameters
`start_from_nth` – `u32` - Represents the index at which to start the sequence.

### `--binet, -b [nth]`

Calculates the nth Fibonacci number using [Binet's formula](https://en.m.wikipedia.org/wiki/Jacques_Philippe_Marie_Binet).

#### Parameters
`nth` – `u32` - Represents the index of the desired Fibonacci term.

## Using the Library

### Iterating through the sequence
```Rust
use fibonacci::sequence::ConsecutiveTerms;

// Creates an iterator that starts the sequence from the 5th term:
let example = ConsecutiveTerms::new(5);

// The subsequent Fibonacci term is calculated by calling the 'next' method:
example.next();

// Or iterate to the end of the sequence within the 'u32' range using a for loop:
for term in example {
    println!("{}", term);
}
```
### Getting the nth term

```Rust
use fibonacci::formula::binet_formula;

println!("{}", binet_formula(10));

```

## Limitations
Currently, Fibonacci numbers are limited to the `u32` range. The highest supported Fibonacci number is `2,971,215,072` (`F(47)`).

## Motivation
This is my first pet project in Rust. It helps me familiarize myself with the language.
