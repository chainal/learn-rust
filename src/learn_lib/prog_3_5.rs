use std::io;

pub fn action() {
    let number = 3;
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}

pub fn action_loop() {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);
}

pub fn action_while() {
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number = number - 1;
    }
    println!("LIFTOFF!!!");
}

pub fn action_for() {
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }
}

pub fn action_for_range() {
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}

pub fn action_f_arr() {
    println!("Please input N: ");
    let mut n = String::new();
    let n = {
        io::stdin().read_line(&mut n).expect("error read line.");
        match n.trim().parse() {
            Ok(num) => num,
            Err(_) => 0
        }
    };
    if n <= 0 {
        println!("input n is not valid (n >= 1)");
        return;
    }

    for i in 1..(n+1) {
        println!("{}", cal_f(i))
    }
}

fn cal_f(n: i32) -> i32 {
    if n == 1 {
        1
    } else if n == 2 {
        1
    } else {
        cal_f(&n - 1) + cal_f(&n - 2)
    }
}