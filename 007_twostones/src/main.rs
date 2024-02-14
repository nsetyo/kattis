use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Error read input");

    let input = input.trim().parse::<u32>().expect("Invalid number");

    if input & 1 == 1 {
        println!("Alice");
    } else {
        println!("Bob");
    }
}
