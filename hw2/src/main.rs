fn main() {
    // let mut val: Vec<i32> = vec![1, 3, 5, 7];
    // println!("{}", fn1(&mut val));
    // println!("{:?}", val);

    let mut val = 1;
    add_two(&mut val);
    println!("{}", val);
}

fn add_two(val: &mut i8){
    *val += 2;
}

// fn fn1(val: &mut Vec<i32>) -> bool{
//     if val[0] == 1{
//         val.push(15);
//         true
//     }
//     else{
//         false
//     }
// }
