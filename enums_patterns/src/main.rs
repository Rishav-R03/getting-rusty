fn find_word(words: Vec<&str>, target: &str)-> Option<usize> {
    for(ind,&item) in words.iter().enumerate() {
        if item == target {
            return Some(ind); // found it
        }
    }
    None // didn't find it
}


fn main() {
    // Here is an example for some and none
    println!("Hello, world!");
    let fruits = vec!["Apple", "Banana", "Orange"];
    match find_word(fruits,"banana") {
        Some(ind) => println!("Found at index: {ind}"),
        None => println!("Not present"),
    }

    // Other ways to handle
    let username = "Alice";
    let custom_photo : Option<&str> = Some("https://example.com/alice.png");
    let no_photo: Option<&str> = None ;

    //1, Unwrap_or()
    //if there is no photo we use a placeholder "default.jpg"
    let display_photo = no_photo.unwrap_or("default.jpg");
    println!("User {} is using photo: {}", username, display_photo);

    //2. ig let  - great for conditional logic
    //we only want to print social message if a custom photo exist
    if let Some(url) = custom_photo {
        println!("Custom photo url: {}",url)
    }else {
        println!("Custom photo not be built");
    }

    //3. unwrap()
    let photo_url = custom_photo.unwrap();
    println!("Photo url: {}",photo_url);
}
