//declaring a module
pub mod module1;
use crate::module1::rectangle::main_rectangle;

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
} //like a tuple but more extensive, allows us to access data by name and not by index

//creating a tuple struct
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);


fn main() {

    main_rectangle();

    //creating an instance of our struct
    let mut user1 = User{
        email: String::from("my@gmail.com"),
        username: String::from("user1123"),
        sign_in_count: 1,
        active: true,
    };

    let name = user1.username;
    user1.username = String::from("wallace123");

    let user2 = build_user(String::from("adebayo@outlook.com"), String::from("Somebody Somewhere"));

    //creating new instances from existing ones
    let user3 = User{
        email: String::from("momo@gmail.com"),
        username: String::from("sadyyy23"),
        ..user2
    };
}

fn build_user(email: String, username: String) -> User{
    // return a new instance of user
    User{
        email,
        username,
        active: true,
        sign_in_count: 1
    }

}
