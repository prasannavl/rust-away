extern crate rand;
extern crate primal;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Hello, world!");

    use std::collections::HashMap;
    let teams  = vec![String::from("Blue"), String::from("Yellow")];
    let mut initial_scores: Vec<String> = std::vec::Vec::new();
    initial_scores.push("10".to_string());

    let i = initial_scores.iter();

    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    let n = rand::thread_rng().gen_range(1, 10);

    println!("Guess the number! ({})", n);

    let mut guess = String::new();
    io::stdin().read_line(&mut guess)
        .expect("failed to read line");

    let guess_number: u32 = guess.trim().parse()
        .expect("not a number!");

    match guess_number.cmp(&n) {
        Ordering::Less    => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal   => println!("You win!"),
    }

    println!("Your number: {}", guess);
    println!("Good bye!");
}