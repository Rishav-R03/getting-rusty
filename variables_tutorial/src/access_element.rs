use std::io;

pub fn access_element_with_index() {
    let arr1 : [u32;5] = [1,2,3,4,5];
    println!("Enter a index to access element");

    let mut idx = String::new();
    io::stdin()
        .read_line(&mut idx)
        .expect("Failed to read line");

    let idx: usize = idx 
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = arr1[idx];

    println!("The value at index {idx} is {element}");
}