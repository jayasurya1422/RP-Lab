// Tuple Creation and Access

fn main() {
    let tup = (100, 3.14, "Rust");
    let (a, b, c) = tup;

    println!("Tuple values: {}, {}, {}", a, b, c);
    println!("Accessing directly: {}, {}, {}", tup.0, tup.1, tup.2);
}
