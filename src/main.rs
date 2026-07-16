use std::io; // Library to input standard input and output

fn execute (choice : i32, number1 : f32, number2 : f32){

    if choice == 1 {
        let out = number1 + number2;
        println!("The sum of {number1} and {number2} is: {out}");
    } 
    
    else if choice == 2 {
        let out = number1 - number2;
        println!("The difference of {number1} and {number2} is: {out}");
    }

    else if choice == 3 {
        let out = number1 * number2;
        println!("The product of {number1} and {number2} is: {out}");
    }

    else if choice == 4 {
        let out = number1 / number2;
        println!("The division of {number1} and {number2} is: {out}");
    }

    else {
        println!("An error occured! Please try again!");
    }
}

fn main() {

    // println!("Guess the Number Game");

    // println!("Please input your guess.");

    // let mut guess = String::new(); // defining a mutable variable guess (mutable variables are the ones that can be changed after assigning)

    // io::stdin()
    //     .read_line(&mut guess)
    //     .expect("Failed to read the line!");
    
    // println!("You guesses : {guess}");

    // Calculator

    let lne = "========================================================================";
    println!("{lne}");
    println!("Rust Calculator");
    println!("{lne}");

    let options = "\n
    
    Choose the operation:

    [1] Add
    [2] Subtract
    [3] Multiply
    [4] Divide

    ";  // By default, the strings are multiline in rust
    
    println!("{options}");
    let mut chosen = String::new();
    let mut x = String::new();
    let mut y = String::new();
    
    io::stdin()
        .read_line(&mut chosen)
        .expect("Failed to read line");

    let choice : i32 = chosen
        .trim()
        .parse()
        .expect("Please enter a valid type!");

    io::stdin()
        .read_line(&mut x)
        .expect("Failed to read line");

    let number1 : f32 = x
        .trim()
        .parse()
        .expect("Please enter a valid type!");

    io::stdin()
        .read_line(&mut y)
        .expect("Failed to read line");

    let number2 : f32 = y
        .trim()
        .parse()
        .expect("Please enter a valid type!");

    println!("{lne}");
    execute(choice, number1, number2);
    }

