

use std::{io};

fn main() {
    let x = get_input();
    let y = get_input();

    let coordinat = [[3, 2], [4, 1]];

    let x = if x > 0 { 1 } else { 0 };
    let y = if y > 0 { 1 } else { 0 };

    println!("{}", coordinat[x][y]);
}

fn get_input() -> i64 {
    let mut val = String::new();

    io::stdin().read_line(&mut val).expect("Error read stdin");

    val.trim().parse::<i64>().expect("Error parsing input")
}
