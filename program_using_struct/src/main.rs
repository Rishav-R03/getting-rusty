mod methods;
mod associated_functions;

use methods::*;
fn main() {
    println!("Welcome to a program written using structs.");

    //1. Method 1 of doing it
    let width : u32 = 30;
    let height: u32= 50;

    //2. Method 2 of doing it
    let rectangle1 = (30, 50);
    println!("The area of the rectangle is {} square pixels.",area(width,height));
    println!("The area of the rectangle is {} square pixels.",area2(rectangle1));

    //Method 3 of doing it
    let r2 = Rectangle {
        width: 20,
        height: 30,
    };
    println!("The area of the rectangle is {} square pixels.",cal_struct(&r2));
    // println!("The rectangle is {r2} square pixels."); // error that std::fmt::Display doesnot implement Rectangle

    //right approach
    println!("The rectangle is {r2:?}.");
    //putting :? specifier inside curly brackets tells println! we want
    // to use an output format called Debug.

    //Circle impl
    let c1 = Circle{ radius: 10.0};
    let area = c1.area();
    println!("The area of the Circle is {area}");

    let my_clock = Clock { hours: 8 };

    // Rust knows my_clock.hours() is the method because of the parentheses
    if my_clock.hours() < 12 {
        println!("It is {} AM", my_clock.hours); // Accessing the field directly
    }

    let mut user1 = crate::associated_functions::User::new("Alice");
    user1.deactivate();
    println!("The activity status of the user {} is: {}",user1.username,user1.active);
}
#[derive(Debug)] // important, position is also important
//The debug trait enables us to print our struct in way that is useful for developers
struct Rectangle{
    width: u32,
    height: u32,
}
fn cal_struct(rectangle: &Rectangle) -> u32 {
    rectangle.height * rectangle.width
}
fn area(width:u32,height:u32)->u32 {
    width*height
}

fn area2(dimensions:(u32,u32))->u32{
    dimensions.0 * dimensions.1
}
