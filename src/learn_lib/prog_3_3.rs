pub fn action() {
    another_function(5);
}

fn another_function(x: i32) {
    println!("The value of x is: {}", x);
}

pub fn action_block() {
    let x = 5;

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

pub fn function_with_return() {
    let x = plus_one(5);

    println!("The value of x is: {}", x);
}

fn plus_one(x: i32) -> i32 {
    x + 1
}