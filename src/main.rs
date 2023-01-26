mod fizzbuzz;
pub use fizzbuzz::*;

fn main() {
    let fbk = FizzBuzz::default() + ("Kazz", 7);
    (0..=100)
        .step_by(7)
        .fizzbuzz_custom(fbk)
        .for_each(|x| println!("{x}"));
}
