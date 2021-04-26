#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    Washington,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

pub fn action() {
    let s = Coin::Quarter(UsState::Alabama);
    println!("coin {}", value_in_cents(s));
    let s = Coin::Quarter(UsState::Alaska);
    println!("coin {}", value_in_cents(s));
    let s = Coin::Quarter(UsState::Washington);
    println!("coin {}", value_in_cents(s));
    let s = Coin::Penny;
    println!("coin {}", value_in_cents(s));
    let s = Coin::Nickel;
    println!("coin {}", value_in_cents(s));
    let s = Coin::Dime;
    println!("coin {}", value_in_cents(s));
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) =>{
            println!("State quarter from {:?}!", state);
            25
        }
    }
}