fn main() {
    // print_phase("print my argument");
    println!("{}", gcd(20, 4));
    println!("{}", mutiple_return_value(true));
    println!("{}", mutiple_return_value(false));
}

// fn print_phase(phrase: &str){
//     println!("{}", phrase);
// }

fn gcd(mut a: u64, mut b: u64) -> u64{
    while a != 0{
        if a < b{
            let c = a;
            a = b;
            b = c;
        }
        a = a % b;
    }
    b
}

fn mutiple_return_value(flag: bool) ->bool {
    if flag == true{
        true
    }
    else{
        false
    }
}
