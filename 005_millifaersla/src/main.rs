use std::io;

fn main() {
    let a = get_input();
    let b = get_input();
    let c = get_input();

    let mut sorted = [&a, &b, &c];

    sorted.sort();

    if sorted[0] == &a {
        println!("Monnei");
    }

    if sorted[0] == &b {
        println!("Fjee");
    }

    if sorted[0] == &c {
        println!("Dolladollabilljoll");
    }
}

fn get_input() -> u64 {
    let mut val = String::new();

    io::stdin().read_line(&mut val).expect("Error read stdin");

    val.trim().parse::<u64>().expect("Error parsing input")
}
