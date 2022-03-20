use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Must provide a single argument, found {}", args.len());
        process::exit(1);
    }
    let n: u128 = args[1].trim().parse().expect("Please type a number");
    println!("{}", fibbonacci(n));
}

fn fibbonacci(n: u128) -> u128 {
    let mut i = 0;
    let mut prev = 0;
    let mut current = 1;
    while i <= n {
        (prev, current, i) = (current, prev + current, i + 1);
    }
    current
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fibbonacci() {
        let sequence: [u128; 7] = [1, 2, 3, 5, 8, 13, 21];
        for (n, &expected) in sequence.iter().enumerate() {
            assert!(fibbonacci(n as u128) == expected);
        }
    }
}