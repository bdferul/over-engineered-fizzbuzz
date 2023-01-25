fn main() {
    fizzbuzz(5, 10);
}

fn fizzbuzz(start: i32, end: i32) {
    let cnd_map = [("Fizz", 3), ("Buzz", 5)];

    for i in start..=end {
        let mut canvas = String::new();

        for (string, value) in cnd_map {
            if i % value == 0 {
                canvas += string;
            }
        }

        if canvas.is_empty() {
            canvas = i.to_string();
        }

        println!("{canvas}");
    }
}
