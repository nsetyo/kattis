use std::{io, u64};

fn main() {
    let n = get_input();
    let m = get_input();

    println!("{}", n % m);
}

fn get_input() -> u64 {
    let mut val = String::new();

    io::stdin().read_line(&mut val).expect("Error read stdin");

    val.trim().parse::<u64>().expect("Error parsing input")
}
