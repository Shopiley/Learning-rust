use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

#[derive(HelloMacro)]
struct Pancakes;

#[derive(HelloMacro)]
struct Cookies;

fn main() {
    // now pancake can access the functions inside the trait - impl HelloMacro for Pancakes
    Pancakes::hello_macro();

    // the beauty of this macro is that, you no longer need to implement the 
    // HelloMacro trait for each struct ( e.g impl HelloMacro for Cookies)
    // simply declare the macro on top of the struct and all the methods in the stuct will be available to you
    Cookies::hello_macro();
}
