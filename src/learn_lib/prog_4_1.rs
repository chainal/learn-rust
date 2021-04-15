pub fn action() {
    let s = String::from("hello");    // s进入作用域
    takes_ownership(s);    // s的值移动到函数里
                                    // 所以这里不再有效
    let x = 5;                  // x进入作用域
    makes_copy(x);      // x应该移动到函数里
                                    // 但i32是Copy的，所以在后面可以继续使用x
}   // 这里，x先移出了作用域，然后是s，但因为s已经被移走
    // 所以不会有特殊的操作

fn takes_ownership(some_string: String) {   // some_string进入作用域
    println!("{}", some_string);
}   // 这里，some_string移出作用域并调用`drop`方法。占用的内存被释放

fn makes_copy(some_integer: i32) {  // some_integer进入作用域
    println!("{}", some_integer);
}    // 这里，some_integer移出作用域。不会有特殊操作

pub fn action_give() {
    let s1 = gives_ownership();
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);

    println!("s1 {}", s1);
    println!("s3 {}", s3);
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}