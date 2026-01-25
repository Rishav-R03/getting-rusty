

pub fn control_flow_in_rust() {
    let number = 5;
    if number < 5 {
        println!("The number is less than 5");
    }else if number > 5 {
        println!("The number is more than 5");
    } else {
        println!("The number is 5");
    }

    let condition = true;
    let number = if condition {5} else {6};
    // let number = if condition {5} else {"five"}; error of type mismatchh
    let number = if condition {"six"} else {"five"};

    println!("Final number is {number}");
}