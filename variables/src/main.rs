fn main() {
    // variables are immutable by default
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    //constants are value that aren't just immutbale by default but always immutable; 
    //declared with keyword const and type must be annotated
    //it's conventional for it to be declared in caps
    //const can be used in the global scope, and let can only be used in a function e.g main()
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;



    //variable shadowing
    // when a varibale name is re-declared/re-assigned
    // important to use the let keyword
    let y = 5;

    let y = y + 1;

    {
        let y = y * 2;
        println!("The value of y in the inner scope is: {y}");
    }

    println!("The value of y is: {y}");

     // we can change the type of the variable the second time since we essentially creating a new variable 
    let spaces = "   ";          // string type
    let spaces = spaces.len();  // number type
}
