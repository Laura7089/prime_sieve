use prime_sieve::Sieve;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Too few args passed!");
    }
    let to_test: u64 = args[1].parse().unwrap();
    match Sieve::new(to_test).lookup(to_test).unwrap() {
        true => println!("{} is prime", to_test),
        false => println!("{} is not prime", to_test),
    };
}
