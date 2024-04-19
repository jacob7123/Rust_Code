fn main() {
    let mut s = String::from("Hello");
    concat_string(s);
}

fn concat_string(s: String){
    let s2 = String::from(" World");
    println!("{}{}", s, s2);
}

fn control_flow(value: i32){
    if value == 1{
        println!("The value is one");
    }
    else if value > 50{
        println!("The value is greater than 50");
    }
    else if value < 50{
        println!("The value is less than 25");
    }else{
        println!("The value is greater than 25 but less than 50");
    }
}
