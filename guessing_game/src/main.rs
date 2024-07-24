use rand::Rng;
use std::io;
use std::cmp::Ordering;
fn main() {
    println!("Hello, please guess a number between 1-100: ");
    let mut guess = String::new();
    let random_number = rand::thread_rng().gen_range(1..=100);
    let mut num_guesses = 0;
    loop {
        io::stdin().read_line(&mut guess).expect("wow you suck");
        num_guesses += 1;
        match guess.cmp(&random_number) {
            Ordering::Less => println!("number is higher, try again."),
            Ordering::Greater => println!("you're not winner, too high, try again."),
            Ordering::Equal => {
                println!("conglaturation, you're winner!");
                break;
            }
        }
    }
    println!("you took {num_guesses} guesses you nerd");
}
