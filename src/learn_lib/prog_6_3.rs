#[derive(Debug)]
enum UsState {
    Alaska,
    Washington,
}

enum Coin {
    Quarter(UsState),
    Dime,
}

pub fn action() {
    let mut count = 0;
    let coin = Coin::Quarter(UsState::Alaska);
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}", state);
    } else {
        count += 1;
    }

    let coin = Coin::Quarter(UsState::Washington);
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}", state);
    } else {
        count += 1;
    }

    let coin = Coin::Dime;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}", state);
    } else {
        count += 1;
    }

    println!("count {}", count);
}