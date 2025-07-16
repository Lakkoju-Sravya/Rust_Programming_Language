fn main() {
    'outer: for i in 1..=10 {
        'inner: for j in 1..=10 {
            print!("{}\t", i * j);
        }
        println!();
    }
}
