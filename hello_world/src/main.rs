extern crate hello_world_lib;
mod greetings;
mod how_you_hold_data_for_operations;
use greetings::japanese;
use how_you_hold_data_for_operations::primitive::scalar::compare;
use how_you_hold_data_for_operations::derived::user_defined::Comp;
use how_you_hold_data_for_operations::primitive::compound::array::multiplier;

fn main() {
    println!("Hello, world!");
    japanese::movies::run();
    japanese::series::run();

    println!("{}", compare(3, 4, Comp::Equals));
    println!("{}", compare(3, 4, Comp::LessThan));
    println!("{}", compare(3, 4, Comp::GreaterThan));

    let arr = [2.5, 7.0, 3.0];
    println!("The multiplication of {:?} is {}", arr, multiplier(&arr));

    hello_world_lib::run();
    hello_world_lib::run2();
    hello_world_lib::run3();

}
