use std::{i64, io};

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Error read line");

    let sum: i64 = input
        .trim()
        .split(' ')
        .collect::<Vec<&str>>()
        .iter()
        .map(|&w| w.parse::<i64>().expect("Invalid number"))
        .sum();

    println!("{sum}");
}
