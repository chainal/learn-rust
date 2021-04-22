#[derive(Debug)]
enum Message {
    Quit,
    Move{x: u32, y: u32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("who am i {:?}", self);
    }
}

pub fn action() {
    let m = Message::Move {x: 1, y: 1};
    m.call();
    let m = Message::Write(String::from("hello"));
    m.call();
    let m = Message::ChangeColor(1, 2, 3);
    m.call();
    let m = Message::Quit;
    m.call();
}