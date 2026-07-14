use std::io; // Library to input standard input and output

fn main() {

    println!("Guess the Number Game");

    println!("Please input your guess.");

    let mut guess = String::new(); // defining a mutable variable guess (mutable variables are the ones that can be changed after assigning)

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read the line!");
    
    println!("You guesses : {guess}");
}