pub fn action() {
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of x,y,z is: {}, {}, {}", x, y, z);
    println!("The value of tup is: {}, {}, {}", tup.0, tup.1, tup.2);

}

pub fn action_panic() {
    let a = [1, 2, 3, 4, 5];
    let index = 3;// panic if index >= 5
    let element = a[index];

    println!("The value of element is {}", element);
}