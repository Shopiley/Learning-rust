use rand::Rng; //bringing in the Rng trait into scope
use std::cmp::Ordering;
use std::io; //importing the io libray from Rust's standard library //Ordering enum with variant: Less, Greater and Equal

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("Guess the number!");

    loop {
        println!("Please input your guess.");

        let mut guess = String::new(); //creates a mutable variable bound to a new, empty instance of a string -
                                       // which is the result of calling the String::new() function

        io::stdin() //calling the stdin function from the io module
            .read_line(&mut guess) //reads user input and appends that to the guess string (without overriding its content)
            // the & indicates that this argument is a reference, which gives you a way to let multiple parts of your code access one piece of data without needing to copy that data into memory multiple times.
            //references are also immutable by default hence the mut
            .expect("Failed to read line"); //the read_line function also outputs a Result value. Result is an enum of two states/variants: ok (contains the successfully generated value) and Err(contains the error message)
                                            // expect is a method that can be called on result type, read more here https://rust-book.cs.brown.edu/ch02-00-guessing-game-tutorial.html
        let guess: u32 = match guess
            .trim() //The trim method on a String instance will eliminate any whitespace at the beginning and end also \n added when a user presses enter
            .parse() {
                Ok(num) => num,
                Err(_) => continue, //underscore is a catch all value; continue here tells the program to go to the next iteration of the loop and ask for another guess
            };
        println!("You guessed: {guess}");

        // println!("The secret number is: {secret_number}");

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
