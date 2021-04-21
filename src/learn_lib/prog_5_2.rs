pub fn action() {
    let width = 30;
    let height = 50;
    println!("The area of the rectangle is {} square pixels.", area(width, height));
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

pub fn action_tuple() {
    let rect = (30, 50);
    println!("The area of the rectangle is {} square pixels.", area_tuple(rect));
}

fn area_tuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

struct Rectangle {
    width: u32,
    height: u32,
}

pub fn action_struct() {
    let rect = Rectangle {width: 30, height: 50};
    println!("The area of the rectangle is {} square pixels.", area_struct(&rect));

}

fn area_struct(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}