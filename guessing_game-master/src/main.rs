use std::io;    //importing the io libray from Rust's standard library

fn main() {
    println!("Guess the number!");
    println!("Please input your guess.");

    let mut guess = String::new(); //creates a mutable variable bound to a new, empty instance of a string -
                                    // which is the result of calling the String::new() function

    io::stdin() //calling the stdin function from the io module
        .read_line(&mut guess)  //reads user input and appends that to the guess string (without overrinding its content)
        // the & indicates that this argument is a reference, which gives you a way to let multiple parts of your code access one piece of data without needing to copy that data into memory multiple times.
        //references are also immutable by default hence the mut
        .expect("Failed to read line"); //the read_line function also outputs a Result value. Result is an enum of two states/variants: ok (contains the successfully generated value) and Err(contains the error message)
        // expect is a method that can be called on result, read more here https://rust-book.cs.brown.edu/ch02-00-guessing-game-tutorial.html
    println!("You guessed: {guess}");
}
