//input taken by user
use std::io;

fn is_prime(n: i32) -> bool {
    if n <= 1 {
        return false;
    }
    if n <= 3 {
        return true;
    }
    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }
    let mut i = 5;
    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }
    true
}

fn main() {
    println!("Enter a number to check if it is prime:");
    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Failed to read line");

    let num: i32 = input.trim().parse()
        .expect("Please type a number!");

    if is_prime(num) {
        println!("{} is a prime number.", num);
    } else {
        println!("{} is not a prime number.", num);
    }
}
