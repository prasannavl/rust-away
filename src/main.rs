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

    // FIXME: autocomplete shows into_iter - but not iter(). It seems to be a partial autocomplete list.
    // FIXME: Type i unresolved, and as such no auto-complete further (possibly due to the above)    
    let i = initial_scores.iter();

    // FIXME: Unresolved type (possibly related again to the above).
    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    let n = rand::thread_rng().gen_range(1, 10);

    println!("Guess the number! ({})", n);

    // FIXME: Type recognition works. But auto-complete is partial - methods like cmp not available.
    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("failed to read line");

    // FIXME: No autocomplete for guess_number even though the type is explicitly provided.
    let guess_number: u32 = guess.trim().parse()
        .expect("not a number!");

    // FIXME: guess.cmp should be available on autocomplete for 'guess', but it isn't. 
    // guess.cmp

    match guess_number.cmp(&n) {
        Ordering::Less    => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal   => println!("You win!"),
    }

    // FIXME: Only displays test mod inside. All other types are missing. This is just an example. Most cargo package don't show up with proper auto-complete. Uncomment and see if autocomplete works.
    // primal::

    println!("Your number: {}", guess);
    println!("Good bye!");
}