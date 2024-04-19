fn main() {
    let mut x = 5;
    println!("The value of x is {}", x);

    x = 6;
    println!("The value of x is {}", x);

    const SECONDS: i8 = 60; // Cannot use mut
    println!("The value of seconds is {}", SECONDS);


}
