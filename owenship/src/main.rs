fn main() {
    // let var = 1; //create on the stack since it is fixed.
    // let mut s = "Jacob".to_string(); //create on the heap because of memory allocation.
    // s.push_str(" Huang"); 

    /*
        Move
     */
    // let x = vec!["tyler".to_string()];
    // let y = x; // x value move to y
    // let z = y; // y value move to z

    // println!("{:?}", x); // fail because x value is moved
    // println!("{:?}", y); // fail because y value is moved
    // println!("{:?}", z);

    /*
        Clone
     */
    // let x = vec!["tyler".to_string()];
    // let y = x.clone(); // x value clone to y
    // let z = y.clone(); // y value clone to z

    // println!("{:?}", x);
    // println!("{:?}", y);
    // println!("{:?}", z);

    /*
        Copy
     */
    // let x = 1; 
    // let y = x;
    // println!("x = {}, y = {}", x, y); // It works since values store in stack.

    /*
        More Moves
     */
    // let s = String::from("Jacob"); // create a variable with a string Jacob
    // takes_ownership(s); // give a owership to the function
    // // println!("{}", s); // s does not have ownership

    // let v = 1;
    // make_copy(v);
    // println!("{}", v);

    // let str1: String = give_ownership();
    // println!("{}", str1);

    // let str3: String = take_and_give(str1);
    // // println!("{}", str1); // It cannot work

    // if(true){
    //     let str4 = str3;
    // }else{
    //     let str5 = str3;
    // }
    // println!("{}", str3); // It cannot work

    // let mut str1 = String::from("Tyler");
    // let mut str2:String;

    // loop{
    //     // str2 = str1; // It cannot work because it is loop. str1 value was moved to str2 in first loop.
    // }

    /*
        Reference and Borrowing
     */
    let mut s = String::from("Hello");
    change_string(&mut s);
    println!("{}", s);
}
// var  is dropped, s is dropped

fn change_string(some_string: &mut String){
    some_string.push_str(", world");
}

// fn takes_ownership(s:String){ 
//     let strin = s;
//     println!("{}", strin);
// }

// fn make_copy(v:i32){ // i32 importments copy
//     let value = v;
//     println!("{}", value);
// }

// fn give_ownership() -> String{ // i32 importments copy
//     "give".to_string()
// }

// fn take_and_give(str2:String) -> String{
//     str2
// }


