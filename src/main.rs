// FIXME: Setting the cursor on the below 'rand' and 'primal'
// words and using "Go-to-definition" should work. 
extern crate rand;
extern crate primal;

// FIXME: Go-to definitions on the the use-statement
// modules/types/traits should take them to 
// the corresponding source.
use std::io;
use std::cmp::Ordering;
use rand::Rng;
use std::io::Write;

fn main() {

    // FIXME: The to_string method should be 
    // available in auto-complete.
    let msg = "Hello, world!\n".to_string();

    // FIXME: The as_bytes method should be
    // available in auto-complete.
    std::io::stdout().write(msg.as_bytes());

    // FIXME: Go-to-definition on HashMap should work
    use std::collections::HashMap;

    // FIXME: Type of "teams" should be recognized and autocomplete on the variable beyond that point
    // should work accordingly.
    let teams  = vec![String::from("Blue"), String::from("Yellow")];

    // Type of "initial_scores" has to be recognized and autocomplete on the variable beyond that point
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