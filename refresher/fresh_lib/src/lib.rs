// trait ===interface in rust

pub fn hey_lib(){
    let intro = String::from("Hey, from fresh_lib");

    println!("{}", intro);

    let some_rect = Rect{ length: 2f32, width: 5f32, name: "some_rect" };

    println!("{:#?}", some_rect);
    println!("{:#?}", Rect::default());
}

#[derive(Debug)]
struct Rect{
    length: f32, 
    width: f32,
    name: &'static str, //represents a string slice with a static lifetime, stored on the stack
}

impl Rect{
    fn default() -> Self{
        Rect {
            length: 1f32,
            width: 1f32,
            name: "Sope",
        }
    }
}

trait Shape {
    // method signatures for area, new objects, getters and setters
    fn area(&self) -> f32;
    fn new(length: f32, width: f32, name: &'static str) -> Self; //an associated function to create a new shape. Not a method because it is not taking in self
    fn get_length(&self) -> f32;
    fn set_length(&mut self, length: f32);
    fn get_width(&self) -> f32;
    fn set_width(&mut self, width: f32);
    fn get_name(&self) -> &'static str;
    fn set_name(&mut self, name: &'static str);
}

impl Shape for Rect {
    fn area(&self) -> f32 {
        self.length * self.width
    }

    fn new(length: f32, width: f32, name: &'static str) -> Self {
        Rect {
            // we can use struct shorthand initialization
            length,
            width,
            name,
        }
    }

    fn get_length(&self) -> f32 {
        self.length
    }

    fn set_length(&mut self, length: f32) {
        self.length = length
    }

    fn get_width(&self) -> f32 {
        self.width
    }

    fn set_width(&mut self, width: f32) {
        self.width = width
    }

    fn get_name(&self) -> &'static str {
        self.name
    }

    fn set_name(&mut self, name: &'static str) {
        self.name = name
    }
}

// Derive an implementation of default triat
#[derive(Debug, Clone)]
struct Rect2{
    length: f32, 
    width: f32,
    name: String, //stored on the heap
}

impl Rect2 {
    fn default() -> Self {
        Rect2 {
            length: 1f32,
            width: 1f32,
            // name: "sope".to_string(),  //converts string slice (&str) to String
            name: String::from("Sope")
        }
        // name: "default_name".to_owned(), //at a time, this was better and more used than to_string..
        //slicing is taking a full string or a string that you can get a part of it, meanin that you want it static. to_string() and to_owned() converts string slice to a heap
        //Demanding for memory for a string is to create a string for it. The compiler would then provide a pointer to that string.
        //If someone else wants it, they will borrow it. If they want to own it, they can to to_owned or create a new String using new string or string from.
                                   
    }
}


trait Shape2 {
    fn area(&self) -> f32;
    // fn new(length: f32, width: f32, name: String) -> Self;
    fn get_length(&self) -> f32;
    fn set_length(&mut self, length: f32);
    fn get_width(&self) -> f32;
    fn set_width(&mut self, width: f32);
    fn get_name(&self) -> &str;     //returning this instead of &String prevents direct access to the String object further preventing the possibility of modification or transfer of ownership
    fn set_name(&mut self, name: &str);
}

impl Shape2 for Rect2 {
    ///Associated function used to create a new shape
    /// Not  amethod because it is not taking in self

    ///Area method
    fn area(&self) -> f32 {
        self.length * self.width
    }

    fn get_length(&self) -> f32 {
        self.length
    }

    fn set_length(&mut self, length: f32) {
        self.length = length;
    }

    fn get_width(&self) -> f32 {
        self.width
    }

    fn set_width(&mut self, width: f32) {
        self.width = width;
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn set_name(&mut self, name: &str) //it can slice/get from anywhere
    {
        self.name = name.to_owned();
    }
}


// don't understand yet ------------------------------------------------------------------
impl PartialEq for Rect {
    fn eq(&self, other: &Self) -> bool {
        // self.length == other.length && self.width == other.width && self.name == other.name;
        self.area() == other.area()
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

// The provided code demonstrates the implementation of the From and Into traits for the Rect2 struct, 
// allowing conversion from a string representation and the Rect2 type and vice versa.

// Implement a From trait for Rect2 that takes a string slice with the format "length, width, name"
impl From<&str> for Rect2 {
    fn from(s: &str) -> Self {
        //when you are treversing an array .next(), you are making changing because you are changing the pointer so therefore it should be mutable
        let mut parts = s.split(',');
        let length = match parts.next() {
            Some(val) => val.parse::<f32>().unwrap(),
            None => 0f32,
        };

        let width = match parts.next() {
            Some(val) => val.parse::<f32>().unwrap(),
            None => 0f32,
        };

        let name = match parts.next() {
            Some(val) => val,
            None => "",
        };

        Rect2 {
            length,
            width,
            name: name.to_owned(),
        }
    }
    //Essentially, we are supposed to tokenize the string and extract the three parts into respective variables.
    //Then use them to constitute a new Rect2
}

//Implement Into trait for Rect2
impl Into<String> for Rect2 {
    // this requires the heap because you are returning String
    fn into(self) -> String {
        //Let's return a string template literal

        //format macro can have the features of print macro but not to specifically print to anywhere
        format!("My name is {} and my area is {}.", self.name, self.area())

        //Print macro is to print to the console or to anywhere
    }
}