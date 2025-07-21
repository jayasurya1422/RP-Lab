// Array Slicing

fn main() {
    let arr = [10, 20, 30, 40, 50, 60, 70, 80, 90, 100];

    let slice1 = &arr[1..3]; 
    let slice2 = &arr[..3];  
    let slice3 = &arr[7..];  
    let slice4 = &arr[..];   

    println!("Slice 1 (2nd and 3rd): {:?}", slice1);
    println!("Slice 2 (start to 3rd): {:?}", slice2);
    println!("Slice 3 (8th to end): {:?}", slice3);
    println!("Slice 4 (entire array): {:?}", slice4);
}
