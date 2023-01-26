mod fizzbuzz;
use std::fmt::Display;

use fizzbuzz::*;

fn main() {
    (1..=100)
        .fizzbuzz_custom(FizzBuzz::default() + ("Kazz", 7))
        .enumerate()
        .filter(|(_, s)| s.parse::<i32>().is_err())
        .map(|(i, s)| format!("{}: {s}", i + 1))
        .print()
        .count();
}

#[must_use = "iterators are lazy and do nothing unless consumed"]
struct PrintIter<I> {
    iter: I,
}

impl<T: Display, I: Iterator<Item = T>> Iterator for PrintIter<I> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let Some(next) = self.iter.next() else { return None };
        println!("{next}");
        Some(next)
    }
}

trait ToPrintIter<I> {
    #[warn(unused_must_use)]
    fn print(self) -> PrintIter<I>;
}

impl<T: Display, I: Iterator<Item = T>> ToPrintIter<I> for I {
    fn print(self) -> PrintIter<I> {
        PrintIter { iter: self }
    }
}
