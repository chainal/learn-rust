
struct User {
    user: String
}

struct Color(i32, i32, i32);


pub fn action() {
    let user = String::from("hello");
    let s = User {
        user
    };
    println!("the value user of User is {}", s.user);
}

pub fn action_color() {
    let black = Color(0, 0, 0);
    println!("the first value of black is {}", black.0);

}