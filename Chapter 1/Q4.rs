// Scope and Shadowing

fn main() {
    let x = 5;
    println!("Outside block: x = {}", x);

    {
        let x = x * 2; 
        println!("Inside block: x = {}", x);
    }

    println!("After block: x = {}", x);
}
