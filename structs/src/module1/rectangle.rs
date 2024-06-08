#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

// let's make an implementation block for a struct which will house the functions and methods associated with our struct
// a method in a struct is tied to an instance of the struct
impl Rectangle{
    // the fist argument of a method is always self
    // self - the instance the method is being called on
    fn area(&self) -> u32{
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool{
        // check if current instance has a greater width and height than the passed in rectangle
        self.width > other.width && self.height > other.height
    }

    // we can also define associated functions here, which are not tied to an instance of our struct
    fn square(size: u32) -> Rectangle{
        Rectangle { width: (size), height: (size) }
    }
}

pub fn main_rectangle() {
    let rect = Rectangle {
        width: 30,
        height: 50,
    };

    let rect1 = Rectangle {
        width: 20,
        height: 40,
    };

    let rect2 = Rectangle {
        width: 40,
        height: 40,
    };

    let rect3 = Rectangle::square(4);

    println!("rect can hold rect1: {}", rect.can_hold(&rect1));
    println!("rect can hold rect2: {}", rect.can_hold(&rect2));

    println!("rect: {:#?}", rect);

    println!("I printed this from rectangle.rs in module1: {}",
    rect.area());
}

