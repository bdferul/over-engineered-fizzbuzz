mod fizzbuzz;
pub use fizzbuzz::*;

fn main() {
    let mut range = 1..=30;
    for i in range.fizzbuzz() {
        println!("{i}");
    }
}
