pub fn greeting_from_lib() -> String {
    let message = String::from("Hello from lib");
    println!("{}", message);
    message
}

// polymorphism === interface
// trait ===interface in rust

trait Shape {
    fn area(&self) -> f32;
    fn new(length: f32, width: f32, name: &'static str) -> Self;
    fn get_length(&self) -> f32;
    fn set_length(&mut self, length: f32);
    fn get_width(&self) -> f32;
    fn set_width(&mut self, width: f32);
    fn get_name(&self) -> &'static str;
    fn set_name(&mut self, name: &'static str);
}

trait Shape2 {
    fn area(&self) -> f32;
    // fn new(length: f32, width: f32, name: &str) -> Self;
    fn get_length(&self) -> f32;
    fn set_length(&mut self, length: f32);
    fn get_width(&self) -> f32;
    fn set_width(&mut self, width: f32);
    fn get_name(&self) -> &str;
    fn set_name(&mut self, name: &str);
}

struct Rect2 {
    length: f32,
    width: f32,
    name: String,
}

impl Rect2 {
    fn defualt() -> Self {
        Rect2 {
            name: String::from("default name"), //"default name".to_string() or to_owned()
            length: 1f32,
            width: 1f32,
        }
    }
}

impl Shape2 for Rect2 {
    ///Associated function used to create a new shape
    /// Not  amethod because it is not taking in self

    // fn new(length: f32, width: f32, name: &str) -> Self {
    //     // If the name of the element is the same as the name of the variable, you can just write it alone like that
    //     Rect2 {
    //         length,
    //         width,
    //         name.to_owned()
    //     }
    // }

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

    fn set_name(&mut self, name: &str) {
        self.name = name.to_owned();
    }
}

//implement a from trait for Rect2 that takes a string slice with the format "length, width, name"
impl From<&str> for Rect2 {
    //Essentially: we are supposed to tokenize the string and extract the three parts into respective variables.
    //then use them to constitute the new Rect 2
    fn from(s: &str) -> Self {
        let mut parts = s.split(','); //when you're iterating through an array, you cause an internal modification bcos you are changing the next() attribute of parts
        let length = match parts.next() {
            Some(val) => val.parse::<f32>().unwrap(),
            None => 0f32,
        };

        let width = match parts.next() {
            Some(val) => val.parse::<f32>().unwrap(),
            None => 0f32,
        };
        let name = match parts.next() {
            Some(val) => val.to_owned(),
            None => "".to_owned(),
        };

        Rect2 {
            length,
            width,
            name, //name.to_owned()
        }
    }
}

//implement into trait for Rect2
impl Into<String> for Rect2 {
    fn into(self) -> String {
        //let's return a string template literal
        // kinda like print but you're formatting
        format!("My name is {} and my area is {}.", self.name, self.area())
    } //this function is going to be on the heap
}

#[derive(
    // Default,
    Debug,
    Clone,
    Copy,
)]
struct Rect {
    length: f32,
    width: f32,
    name: &'static str,
}

impl Rect {
    fn default() -> Self {
        Rect {
            length: 1f32,
            width: 1f32,
            name: "default_name",
        }
    }
}

impl Shape for Rect {
    ///Associated function used to create a new shape
    /// Not  amethod because it is not taking in self

    fn new(length: f32, width: f32, name: &'static str) -> Self {
        // If the name of the element is the same as the name of the variable, you can just write it alone like that
        Rect {
            length,
            width,
            name,
        }
    }

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

    fn get_name(&self) -> &'static str {
        self.name
    }

    fn set_name(&mut self, name: &'static str) {
        self.name = name;
    }
}

impl PartialEq for Rect {
    fn eq(&self, other: &Self) -> bool {
        // self.length == other.length && self.width == other.width && self.name == other.name;
        self.area() == other.area()
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

pub fn run() {
    let rectangle1: Rect = Rect {
        length: 2.4,
        width: 6.3,
        name: "Rectangle",
    };

    let mut rectangle2: Rect = Rect::default();
    rectangle2.set_length(10f32);
    rectangle2.set_width(5f32);

    let rectangle3 = rectangle1.clone();

    let rectangle4 = Rect { ..rectangle1 };

    let rectangle5 = Rect {
        length: 12f32,
        ..rectangle1
    };

    println!("rectangle 1 is {:#?}", rectangle1);
    println!("rectangle 2 is {:#?}", rectangle2);
    println!(
        "Area of rectangle 1 is {}",
        format!("{:.2?}", rectangle1.area())
    );

    assert_eq!(rectangle1, rectangle3);
    println!("If you can see this, your two triangles are equal");
}

// let's demand that our struct be created on the heap
// we will need to use a smart pointer: the box
pub fn run2() {
    let mut rectangle1 = Box::new(Rect2 {
        //Box<dyn Shape2>
        length: 12f32,
        width: 9f32,
        name: "Rectangle 1".to_owned(),
    });

    println!("Area = {}", rectangle1.area());

    let rectangle2 = Rect2::from("20.0, 30.0, Rectangle2");
    let rectangle2: Rect2 = "20.0, 30.0, Rectangle2".into();
}

pub fn run6() {
    let mut x: u32 = 10;

    println!("x before change = {}", x);

    let y: &mut u32 = &mut x; //y is a mutable reference to x
    let z: *const u32 = y; //z is an immutable raw pointer to y which references x
                           // let a = y as *mut u32;   //a is a mutable raw pointer to y which references x
    let a: *mut u32 = y; //a is also a mutable raw pointer to y which references x

    // & gets the value while * gets the address
    println!("y = {:?}", y); //expect value in x
    println!("z = {:?}", z); //expect memory address of y
    println!("a = {:?}", a); //expect same memory address as z above

    *y = 11; //expect value in x to change
    println!("x after first change ={}", x);

    unsafe {
        *a = 12; //expect value in x to change
        assert!(x == 12)
    }
    println!("x after second change ={}", x);
}

//Error Handling----------------------------------------------------------------------------------------------------------------------------
pub fn run7() {
    //Error handling
    // panic!("You called panic");

    //Illustraction Some
    let mut v = vec!["a", "b", "c"];

    //pop an element from the vector
    let x = v.pop();

    //if by your business value, you are sure that vec contains values i.e Option is Some, use unwrap to bring out the value
    // expect does thesame thing as unwrap then it also provides an error message should incase there is no value to unwrap
    println!("{}", x.unwrap());
    println!("{}", x.expect("I expected a value from my vec"));

    //what if we know there's a possiblity of having no Some value but we don't want to crash if we don't get a value
    match x {
        Some(value) => println!("we've got a value"),
        None => println!("Your vector is empty"),
    }

    //compare above to
    let mut v2: Vec<&str> = Vec::new();

    //continue from phone

    //let's use ? for option
    let mut v3 = vec![1, 2, 3];

    //Closures(read up more)
    //   - don't have to have names
    //   - within a given scope where you define a function, that func cannot use variable defined
    //     outside it unless they were passed to it. Closure can do this
    let mut plus_one = || -> Option<i32> { Some(v3.pop()? + 1) };

    println!("Plus one {}", plus_one().unwrap())

    //Let's see Result instead of Option
    // here it returns Ok value vs Err, unlike Option that returns Some value
}

//Adjust the following to return Result
pub fn multiplier(numbers: &[f64]) -> f64 {
    let mut product = 1f64;

    for n in numbers {
        product = product * n;
    }

    product
}

//what if we want to return Err to the caller of this function when less then two arguments are  passed

#[derive(Debug)]
pub struct ErrorTypes {
    pub number: u8,
    pub message: &'static str,
    pub detail: &'static str,
}

//let's create static variables for our error types
const INVALID_ARG_LEN_ERR: ErrorTypes = ErrorTypes {
    number: 101,
    message: "Invalid Arg Length",
    detail: "Two or more aguments are expected",
};

const INVALID_ARG_TYPE_ERR: ErrorTypes = ErrorTypes {
    number: 102,
    message: "Invalid Arg Type. f64 expected",
    detail: "Invalid Arg Type. f64 expected. You must convert your arg to f64",
};

pub fn mature_multiplie(numbers: &[f64]) -> Result<f64, ErrorTypes> {
    if numbers.len() < 2 {
        return Err(INVALID_ARG_LEN_ERR);
    }

    let mut product = 1f64;

    for n in numbers {
        product *= n;
    }

    Ok(product)
}

/*
Case 1: We would like to create a macro that would allow us instantiate
one or more rectangles (along with their Shape trait impl) at a go. i.e., :
rectangles!((length:f32, width:f32, name:&str),…,n)
E.g., rectangles!((1, 1, "rect1"), (3.5, 7.0, "rect2"))
 */

//declare a macro export, in-built in rust (no_std)
#[macro_export]
macro_rules! rectangles{  //declarative macro
    ($($rect_props_tuple: expr), *) => {

        //I want to return a vector of rectangles (created with the dimensions given)
        {
            let mut rect_vec = Vec::new();

            //take our expr received, get the length, width and name from each and create the appropriate rectangle and push each to our vector - rect_vec

            $(let (length, width, name) = $rect_props_tuple;
            let rect = Rect{length: length as f32, width: width as f32, name: name as &str};
            rect_vec.push(rect);
            )*

            rect_vec
        }
    };
}

//Try our rectangles! declarative macro
pub fn run9() {
    let rects = rectangles!((1, 1, "rect1"), (3.5, 7.0, "rect2"));
    println!(
        "Area of rectangle 1 = {}; area of the rectnangle 2 = {}",
        rects[0].area(),
        rects[1].area(),
    )
}

//You can also have multiple expressions in a declarative macro.
//what if you want a second expression that contains defaults for
//length, width, and name for the rect
// this implies that length, width and name will be optional

#[macro_export]
macro_rules! rectangles_with_default {
    (($($rect_props_tuple: expr), *), $default_props_tuple:expr) => {
        {
            let mut rect_vec = Vec::new();

            let (default_length, default_width, default_name) = $default_props_tuple;
            $(
                let (length, width, name) = $rect_props_tuple;

                let rect = Rect{
                    length: if length.is_none(){default_length as f32} else {length.unwrap() as f32},
                    width: if width.is_none(){default_width as f32} else {width.unwrap() as f32},
                    name: if name.is_none(){default_name as &str} else {name.unwrap() as &str}
                };
                rect_vec.push(rect);
            )*
            rect_vec
        }
    };
}

pub fn run10() {
    let rects = rectangles_with_default!(
        (
            (None as Option<i32>, Some(1), Some("rect1")),
            (Some(3.5), Some(7.0), None as Option<&str>)
        ),
        (1, 1, "default_name")
    );
    println!(
        "Area of rectangle 1 names '{}' = {}; rectangle 2 named '{}' area = {}",
        rects[0].name,
        rects[0].area(),
        rects[1].name,
        rects[1].area()
    );
}

pub fn run11() {
    use rect_shape::Shape;
    use rect_shape_derive::Shape;

    #[derive(Debug, Clone, Shape)]
    struct RectWithDerivedShape {
        length: f32,
        width: f32,
        name: &'static str,
    }

    //the Shape trait implementations
    //should be available for RectWithDerivedShape
    //without further explicit implementation

    let rectangle1 = RectWithDerivedShape {
        length: 1.0,
        width: 2.0,
        name: "Rect 1 with derived Shape trait",
    };
    println!(
        "Area of rectangle1 with derived Shape = {}",
        rectangle1.area()
    );
}

pub fn run12() {
    //next is Attribute-like macros
    use attribute_macros_lib::my_attribute_macro;

    #[my_attribute_macro(7)]
    fn my_ordinary_function(x: i32) -> i32 {
        x * 3
    }

    println!("{}", my_ordinary_function(3))
}

pub fn run13() {
    //next is Attribute-like macros
    use attribute_macros_lib::route;

    #[route("GET", "/")]
    fn my_controller_endpoint() -> &str {
        let header = "test header";
        header
    }

    println!("{:?}", my_controller_endpoint())
}
