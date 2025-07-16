fn main() {
    let mut count = 0;
    let mut number = 0;

    loop {
        count += 1;
        number += 7;

        println!("Iteration: {}, Number: {}", count, number);

        if number >= 50 {
            break;
        }
    }

    println!("Total iterations until number >= 50: {}", count);
}
