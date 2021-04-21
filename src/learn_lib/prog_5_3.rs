struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.height * self.width
    }
}

pub fn action() {
    let rect = Rectangle { width: 30, height: 50};
    println!("The area of the rectangle is {} square pixels.", rect.area());
}