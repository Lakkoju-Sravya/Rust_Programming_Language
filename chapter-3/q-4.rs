fn main() {
    let a = ("Mamidi", "Pavani","CSE");

    if let ("Mamidi", "Pavani","CSE") = a {
        println!("Wrote all values in pattern to be matched with the scrutinee expression");
    } else {
     
        println!("Value unmatched");
    }
}
