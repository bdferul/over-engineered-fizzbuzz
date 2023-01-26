use std::{
    fmt::Debug,
    ops::{Add, AddAssign},
};

#[derive(Clone)]
struct Param {
    pub string: String,
    pub value: i32,
}

impl Param {
    pub fn new(string: &str, value: i32) -> Param {
        Self {
            string: string.to_string(),
            value,
        }
    }

    pub fn tuple(self) -> (String, i32) {
        (self.string, self.value)
    }
}

impl From<(&str, i32)> for Param {
    fn from(value: (&str, i32)) -> Self {
        Self::new(value.0, value.1)
    }
}

impl Debug for Param {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", (&self.string, self.value))
    }
}

/// An over-engineered [FizzBuzz] implementation.
/// # Usage
/// - Basic implementation
/// ```
/// # use over_engineered_fizzbuzz::*;
/// let fb = FizzBuzz::default();
///
/// for i in 1..100 {
///     let res = fb.compute(i);
///     println!("{res}");
/// }
/// ```
/// - Add Paramater
/// ```
/// # use over_engineered_fizzbuzz::*;
/// let mut fb = FizzBuzz::default();
/// fb = fb + ("Kazz", 7);
/// fb += ("Vezz", 11);
///
/// let left = fb.compute(3 * 5 * 7 * 11);
/// let right = "FizzBuzzKazzVezz";
///
/// assert_eq!(left, right);
/// ```
#[derive(Clone, Debug)]
pub struct FizzBuzz {
    params: Vec<Param>,
}

impl FizzBuzz {
    /// Checks to see if the given input is divisible by any of its stored paramaters.
    /// # Usage
    /// ```
    /// # use over_engineered_fizzbuzz::*;
    /// let fb = FizzBuzz::default();
    /// assert_eq!(fb.compute(15), "FizzBuzz");
    /// ```
    pub fn compute(&self, a: i32) -> String {
        compute_fizzbuzz(self, a)
    }

    /// Perfoms [compute_fizzbuzz] on all items in an iterator
    /// # Usage
    /// ```
    /// # use over_engineered_fizzbuzz::*;
    /// let left = FizzBuzz::default().compute_iter(1..=5);
    /// let right = ["1", "2", "Fizz", "4", "Buzz"];
    /// assert_eq!(left, right);
    /// ```
    pub fn compute_iter<I: Iterator<Item = i32>>(&self, iter: I) -> Vec<String> {
        iter.map(|x| self.compute(x)).collect()
    }

    /// Returns a [FizzBuzz] struct with `0` paramaters
    /// # Usage
    /// ```
    /// # use over_engineered_fizzbuzz::*;
    /// let left = FizzBuzz::empty().params();
    /// let right = [];
    ///
    /// assert_eq!(left, right);
    /// ```
    pub fn empty() -> Self {
        FizzBuzz::from(vec![])
    }

    /// Returns the [Vec] of stored paramaters
    /// # Usage
    /// ```
    /// # use over_engineered_fizzbuzz::*;
    /// let left = FizzBuzz::default().params();
    /// let right = [("Fizz".to_string(), 3), ("Buzz".to_string(), 5)];
    /// assert_eq!(left, right);
    /// ```
    pub fn params(&self) -> Vec<(String, i32)> {
        self.params.iter().map(|p| p.clone().tuple()).collect()
    }
}

impl Default for FizzBuzz {
    /// The [Default] paramaters are set as the paramaters given in the original interview question.
    ///
    /// Fizz maps to `3`, and Buzz maps to `5`.
    /// # Usage
    /// ```
    /// # use over_engineered_fizzbuzz::*;
    /// let left = FizzBuzz::default().params();
    /// let right = [("Fizz".to_string(), 3), ("Buzz".to_string(), 5)];
    /// assert_eq!(left, right);
    /// ```
    fn default() -> Self {
        Self::from(vec![("Fizz", 3), ("Buzz", 5)])
    }
}

impl From<Vec<(&str, i32)>> for FizzBuzz {
    /// # Usage
    /// ```
    /// # use over_engineered_fizzbuzz::*;
    /// let fb = FizzBuzz::from(vec![("Fizz", 3), ("Buzz", 5)]);
    /// ```
    fn from(value: Vec<(&str, i32)>) -> Self {
        Self {
            params: value.into_iter().map(|(s, x)| Param::new(s, x)).collect(),
        }
    }
}

impl AddAssign<Param> for FizzBuzz {
    fn add_assign(&mut self, rhs: Param) {
        self.params.push(rhs);
    }
}

impl AddAssign<(&str, i32)> for FizzBuzz {
    /// # Add Paramater
    /// ```
    /// # use over_engineered_fizzbuzz::*;
    /// let mut fb = FizzBuzz::default();
    /// fb = fb + ("Kazz", 7);
    /// fb += ("Vezz", 11);
    ///
    /// let left = fb.compute(3 * 5 * 7 * 11);
    /// let right = "FizzBuzzKazzVezz";
    ///
    /// assert_eq!(left, right);
    /// ```
    fn add_assign(&mut self, rhs: (&str, i32)) {
        self.params.push(Param::from(rhs));
    }
}

impl Add<Param> for FizzBuzz {
    type Output = Self;
    fn add(self, rhs: Param) -> Self::Output {
        let mut clone = self.clone();
        clone += rhs;
        self
    }
}

impl Add<(&str, i32)> for FizzBuzz {
    type Output = Self;
    /// # Add Paramater
    /// ```
    /// # use over_engineered_fizzbuzz::*;
    /// let mut fb = FizzBuzz::default();
    /// fb = fb + ("Kazz", 7);
    /// fb += ("Vezz", 11);
    ///
    /// let left = fb.compute(3 * 5 * 7 * 11);
    /// let right = "FizzBuzzKazzVezz";
    ///
    /// assert_eq!(left, right);
    /// ```
    fn add(self, rhs: (&str, i32)) -> Self::Output {
        let mut clone = self;
        clone += rhs;
        clone
    }
}

/// Turn any [i32] [Iterator] into a [FizzBuzzIter]
pub trait FizzBuzzed<I> {
    /// # Usage
    /// ```
    /// # use over_engineered_fizzbuzz::*;
    /// let left = (1..=5).fizzbuzz().collect::<Vec<String>>();
    /// let right = ["1", "2", "Fizz", "4", "Buzz"];
    ///
    /// assert_eq!(left, right);
    /// ```
    fn fizzbuzz(self) -> FizzBuzzIter<I>;

    /// # Usage
    /// ```
    /// # use over_engineered_fizzbuzz::*;
    /// let fb = FizzBuzz::from(vec![("Carl", 3), ("Wheezer", 4)]);
    ///
    /// let left = (1..=5).fizzbuzz_custom(fb).collect::<Vec<String>>();
    /// let right = ["1", "2", "Carl", "Wheezer", "5"];
    ///
    /// assert_eq!(left, right);
    /// ```
    fn fizzbuzz_custom(self, fb: FizzBuzz) -> FizzBuzzIter<I>;
}

impl<I> FizzBuzzed<I> for I
where
    I: Iterator<Item = i32>,
{
    fn fizzbuzz(self) -> FizzBuzzIter<I> {
        self.fizzbuzz_custom(Default::default())
    }

    fn fizzbuzz_custom(self, fb: FizzBuzz) -> FizzBuzzIter<I> {
        FizzBuzzIter::new(fb, self)
    }
}

/// Given a [FizzBuzz] (`fb`) and an [i32] (`var`), returns a [String] of all paramaters in `fb` where `var` is divisible by them.
///
/// If `var` is not divisible by any of the paramaters in `fb`, will return `var.to_string()`
/// # Usage
/// ```
/// # use over_engineered_fizzbuzz::*;
/// let fb = FizzBuzz::default();
///
/// assert_eq!(compute_fizzbuzz(&fb, 1), "1");
/// assert_eq!(compute_fizzbuzz(&fb, 2), "2");
/// assert_eq!(compute_fizzbuzz(&fb, 3), "Fizz");
/// assert_eq!(compute_fizzbuzz(&fb, 5), "Buzz");
/// assert_eq!(compute_fizzbuzz(&fb, 15), "FizzBuzz");
/// ```
pub fn compute_fizzbuzz(fb: &FizzBuzz, var: i32) -> String {
    // le painting
    let mut canvas = String::new();

    for Param { string, value } in &fb.params {
        if var % value == 0 {
            canvas += string;
        }
    }

    if canvas.is_empty() {
        var.to_string()
    } else {
        canvas
    }
}

/// An [Iterator] for generating [FizzBuzz] values from any [i32] [Iterator]
/// # Usage
/// ```
/// # use over_engineered_fizzbuzz::*;
/// let fbi = FizzBuzzIter::new(FizzBuzz::default(), 1..=5);
///
/// let left = fbi.collect::<Vec<String>>();
/// let right = ["1", "2", "Fizz", "4", "Buzz"];
///
/// assert_eq!(left, right);
/// ```
#[derive(Clone, Default, Debug)]
pub struct FizzBuzzIter<I> {
    fb: FizzBuzz,
    iter: I,
}

impl<I: Iterator<Item = i32>> FizzBuzzIter<I> {
    /// # Usage
    /// ```
    /// # use over_engineered_fizzbuzz::*;
    /// let fbi = FizzBuzzIter::new(FizzBuzz::default(), 1..=5);
    ///
    /// let left = fbi.collect::<Vec<String>>();
    /// let right = ["1", "2", "Fizz", "4", "Buzz"];
    ///
    /// assert_eq!(left, right);
    /// ```
    pub fn new(fb: FizzBuzz, iter: I) -> FizzBuzzIter<I> {
        FizzBuzzIter { fb, iter }
    }
}

impl<I: Iterator<Item = i32>> Iterator for FizzBuzzIter<I> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|x| self.fb.compute(x))
    }
}
