3. Ownership Transfer Between Functions


fn take_ownership(s: String) {
    println!("Got ownership of: {}", s);
}

fn main() {
    let s = String::from("Hello Ownership");
    take_ownership(s); // ownership moved here
    // println!("{}", s); // ❌ Error! s is no longer valid
}
