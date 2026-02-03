

pub fn calculate_length(s: &String)-> usize {
    s.len()
}

pub fn calculate_string_length(s: String) -> usize {
    let s1 = &s;
    s1.len()
}

pub fn mutable_length(s: &mut String) -> usize{
    s.len()
}