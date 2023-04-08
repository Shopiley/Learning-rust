use std::io;

fn main() {
    println!("Guess the number!");
    println!("Please input your guess.");

    let mut guess = String::new(); //creates a mutable variable bound to a new, empty instance of a string

    io::stdin() //calling the stdin function from the io module
        .read_line(&mut guess)  //reads user input and append that to the guess string (without overrinding its content)
        // the & indicates that this argument is a reference, references are also immutable by default hence the mut
        .expect("Failed to read line");

    println!("You guessed: {guess}");
}
