//1. Declare that the module exist
mod access_element;

//2. Bring the function into scope
use access_element::access_element_with_index;

fn sample_function() {
    // no parameters
    let x = 5;
    println!("The value of x is {x}");
}

fn parameter_function(x: u32) {
    println!("The parameter entered is: {x}");
}
fn parameter_function_2(x: u32, unit_label: char) {
    println!("The parameter entered has unit: {unit_label} with value {x}");
}

fn main() {
    println!("Variables tutorial");
    let a = 5;
    println!("The value of a is: {a}");

    //a = 6; // error: cannot assign twice to immutable variable
    println!("The updated value of a is {a}");

    let mut b = 10;
    println!("The value of b is {b}");
    b = 11;
    println!("The updated value of b is {b}");

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("{THREE_HOURS_IN_SECONDS}");

    //1. Shadowing to perform calculation
    let shadow1 = 5;
    let shadow1 = shadow1 + 5;

    {
        //2. Shadowing within inner scope
        let shadow1 = shadow1 * 2;
        println!("The value of shadow1 in the inner scope is: {shadow1}");
    }
    println!("The value of shadow1 in the global scope is: {shadow1}");

    //3. Shadowing to change data type
    let spaces = " "; // string
    let spaces = spaces.len(); // this is numerical type (usize)

    println!("Number of spaces : {spaces}");

    // access_element_with_index();

    sample_function();

    parameter_function(6);
    parameter_function_2(6, 'h');
}
