extern crate fresh_lib;
use fresh_lib::hey_lib;
mod school;
use school::sst;
mod how_to_hold_data_for_operations;
use how_to_hold_data_for_operations::primitive::scalar::compare;
use how_to_hold_data_for_operations::derived::user_defined::Comp;
use how_to_hold_data_for_operations::primitive::compound::array::multiplier;

fn main() {
    println!("Hello, world!");
    sst::compsci::run();
    sst::elect::run();

    print!("{}", compare(3, 5, Comp::LessThan));

    let array = [2.0, 6.0, 7.0, 4.0];
    println!("The multiplication of {:?} is {}", array, multiplier(&array));

    hey_lib();
}
