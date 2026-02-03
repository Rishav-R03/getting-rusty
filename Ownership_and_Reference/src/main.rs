mod borrowing_and_references;

use borrowing_and_references::calculate_length;
use borrowing_and_references::calculate_string_length;
use borrowing_and_references::mutable_length;
fn main() {
    println!("Welcome to Ownership and Reference Tutorial");

    //1. Predict if code compiles, if not then correct it
    let s1 = String::from("Hello");
    let s2 = s1;
    // println!("{}, world!",s1);  the code won't work fine as value is moved
    println!("{}, world!",s2); // the code works fine

    //2. Mutable borrow limits
    let mut s = String::from("Hello");
    {
        let r1 = &mut s; // borrowing
        r1.push_str(", world! Learning rust");
        println!("{}",r1);
    }// let r2 = &mut s; throws error as s cannot be borrowed more than once.
    //solution is to use curly braces (scopes)
    let r2 = &mut s;
    // let r3 = s; cannot move out of s because s is borrowed
    println!("{}",r2);

    let len = calculate_length(&s);
    println!("{}",len);

    // let reference_to_nothing = dangle();
    mixed_immutability();

    let mut s4 = String::from("I am learning ");
    println!("{}",s4);
    push_rust(&mut s4);
    println!("{}",s4);

    //use string length function
    // use the function
    let length = calculate_length(&s4); //we don't have to create new ownership
    //because we are borrowing right here.
    println!("{}",length);
    let length2 = calculate_string_length(s4); //we are moving ownership to the function
    //and if we try to access s4 from here we will get error that s4 is moved.
    println!("{}",length2);
    //third kind of implementation using mut
    let mut s5 = String::from("I love Rust");
    let length3 = mutable_length(&mut s5);
    println!("{}",length3);
}
// dangling reference

// fn dangle() -> &String {
//     let s = String::from("Hello");
//     // &s; with throw error. s is dropped at the end of dangle(). You
//     //can't return reference to a dead value
//
//     String
// }

// Mixed immutability

fn mixed_immutability() {
    let mut s = String::from("Hello");
    let r1 = &s;
    let r2 = &s;
    println!("{} and {}", r1, r2);
    let r3 = &mut s;
    println!("{} r3: ", r3);
}

// mutable borrowing
fn push_rust(s:&mut String){
    s.push_str(" rust");
}