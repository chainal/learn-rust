pub fn action() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

pub fn action_mut() {
    let mut s = String::from("hello");
    change(&mut s);

    println!("s = {}", s)
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}