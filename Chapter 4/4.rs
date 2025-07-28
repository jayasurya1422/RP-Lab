4. Ownership and Borrowing Scenarios
  
fn borrow_string(s: &String) {
    println!("Borrowed: {}", s);
}

fn mutable_borrow_string(s: &mut String) {
    s.push_str(" World");
    println!("Mutably borrowed: {}", s);
}

fn main() {
    let s = String::from("Hello");
    borrow_string(&s); // Borrowing works

    let mut s2 = String::from("Hello");
    mutable_borrow_string(&mut s2); // Mutable borrowing works

    // println!("{}", s);  // Still valid because it was only borrowed
}
