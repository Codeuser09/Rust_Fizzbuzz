const FIZZ: i32 = 10;
const BUZZ: i32 = 5;
const NUMBER_RANGE: i32 = 500;

fn fizzbuzz_fn() {
    for number in 1..=NUMBER_RANGE {
        if number % FIZZ == 0 && number % BUZZ == 0 {
            println!("FizzBuzz");
        } else if number % FIZZ == 0 {
            println!("Fizz");
        } else if number % BUZZ == 0 {
            println!("Buzz");
        } else {
            println!("{number}");
        }
    }
}

fn main() {
    fizzbuzz_fn();
}
