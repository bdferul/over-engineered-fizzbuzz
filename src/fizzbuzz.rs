use std::ops::AddAssign;

#[derive(Clone)]
struct Param {
    string: String,
    value: i32,
}

impl Param {
    pub fn new(string: &str, value: i32) -> Param {
        Self {
            string: string.to_string(),
            value,
        }
    }
}

impl From<(&str, i32)> for Param {
    fn from(value: (&str, i32)) -> Self {
        Self::new(value.0, value.1)
    }
}

#[derive(Clone)]
pub struct FizzBuzz {
    params: Vec<Param>,
}

impl FizzBuzz {
    pub fn compute(&self, a: i32) -> String {
        let mut canvas = String::new();

        for Param { string, value } in &self.params {
            if a % value == 0 {
                canvas += string;
            }
        }

        if canvas.is_empty() {
            canvas = a.to_string();
        }

        canvas
    }

    pub fn compute_iter<I: Iterator<Item = i32>>(&self, iter: I) -> Vec<String> {
        iter.map(|x| self.compute(x)).collect()
    }

    pub fn add_param(&mut self, string: &str, value: i32) {
        self.params.push(Param::new(string, value));
    }
}

impl Default for FizzBuzz {
    fn default() -> Self {
        Self::from(vec![("Fizz", 3), ("Buzz", 5)])
    }
}

impl From<Vec<(&str, i32)>> for FizzBuzz {
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
    fn add_assign(&mut self, rhs: (&str, i32)) {
        self.params.push(Param::from(rhs));
    }
}

pub trait FizzBuzzed {
    fn fizzbuzz(&mut self) -> Vec<String>;
    fn fizzbuzz_custom(&mut self, fb: FizzBuzz) -> Vec<String>;
}

impl<I> FizzBuzzed for I
where
    I: Iterator<Item = i32>,
{
    fn fizzbuzz(&mut self) -> Vec<String> {
        let fb = FizzBuzz::default();
        self.fizzbuzz_custom(fb)
    }

    fn fizzbuzz_custom(&mut self, fb: FizzBuzz) -> Vec<String> {
        self.map(|x| fb.compute(x)).collect()
    }
}
