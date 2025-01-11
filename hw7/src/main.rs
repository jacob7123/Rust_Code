use std::rc::Rc;
fn main() {
    /*
     * Q1: Create a variable on the stack and a variable on the heap. Multiply their values and print out the results.
     */

    let x = 10;
    let y = Box::new(20);
    let z = x * *y;
    println!("{}", z);

    /*
     * Q2: Create a variable that holds a String. Then create a reference counting smart pointer that contains the String. 
     *     Print out how many references the smart pointer has. Now inside the code block create another reference counting 
     *     smart pointer that points to our first smart pointer. Print out how many references each smart pointer has.
     */
    let str = String::from("Jacob");
    let s1  = Rc::new(str);
    println!("First Reference count: {}", Rc::strong_count(&s1));
    let s2  = Rc::new(s1.clone());
    println!("Second Reference count: {} and {}", Rc::strong_count(&s1), Rc::strong_count(&s2));

}
