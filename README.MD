# Over-Engineered-FizzBuzz.

An over-engineered FizzBuzz implementation.

# Usage
- Use as an iterator
```rust
for i in (1..=5).fizzbuzz() {
    println!("{i}");
}

/* OUTPUT
1
2
Fizz
4
Buzz
*/
```

- Add Paramater
```rust
let mut fb = FizzBuzz::default();

fb = fb + ("Kazz", 7);
fb += ("Vezz", 11);

let left = fb.compute(3 * 5 * 7 * 11);
let right = "FizzBuzzKazzVezz";

assert_eq!(left, right);
```

- Give custom paramaters
```rust
let fb = FizzBuzz::from(vec![("Carl", 3), ("Wheezer", 4)]);

let left = (1..=5).fizzbuzz_custom(fb).collect::<Vec<String>>();
let right = ["1", "2", "Carl", "Wheezer", "5"];

assert_eq!(left, right);
```

- Basic implementation
```rust
let fb = FizzBuzz::default();

for i in 1..100 {
    let res = fb.compute(i);
    println!("{res}");
}
```