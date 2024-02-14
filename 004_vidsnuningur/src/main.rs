use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Error read input");

    let output: String = input.chars().rev().collect();

    println!("{}", output);
}
