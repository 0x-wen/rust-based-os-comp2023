use std::io;

use rand::Rng;
use std::cmp::Ordering;
fn main() {
    let arr2 = [2; 32];
    let s1 = "world";
    println!("{:?}, {s1}", arr2);

    // xs is array copy
    let xs = [0, 1, 2, 3, 4];
    // Loop through elements in a slice of `xs`.

    for x in xs {
        println!("{}", x);
    }
    println!("later: {:?}", xs);

    let xs1 = vec![1, 2, 3];
    // References must be used here
    for x in &xs1 {
        println!("{}", x);
    }
    println!("later2: {:?}", xs1);

    let secret_number = rand::thread_rng().gen_range(1..=100);
    loop {
        println!("Please input your guess.");
        // Define a mutable variable to use  receive user input
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("failed read line");
        println!("You guessed: {guess}");
        // shadowing
        // let guess: u32 = guess.trim().parse().expect("Please type a number!");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        // guess cmp secret_number
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
