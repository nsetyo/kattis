use std::io;

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Error read the line");

    let mut repeat = String::new();

    io::stdin()
        .read_line(&mut repeat)
        .expect("Error read the line");

    let repeat: usize = repeat.trim().parse().expect("Invalid number");

    println!("{}", input.trim().repeat(repeat));
}
