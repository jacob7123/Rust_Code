use std::rc::Rc;
use std::cell::{Ref, RefCell};


struct Flagger{
    is_true: RefCell<bool>,
}
fn main() {
    /*
        Box is a smart pointer in Rust.
        Finally, b is an address stored on the stack, but tmp moved to heap.
     */
    let tmp = (5, "Jacob"); 
    let b = Box::new(tmp);  
    println!("{:?}", b);

    let x = 5;
    let y = &x;
    // println!("{:p}", y);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    /*
        Reference Counting Pointer
     */

    let s1 = Rc::new(String::from("Jacob"));
    let s2 = s1.clone();
    let s3 = s2.clone();

    println!("{}, {}, {}", s1, s2, s3);
    println!("Reference count: {}", Rc::strong_count(&s1));



    /*
        RefCell
     */

    let flag = Flagger{is_true:RefCell::new(true)};
    // borrow return Ref<T>
    // borrow_mut return RefMut<T>

    // let reference = flag.is_true.borrow();
    // println!("{}", reference);
    // println!("{:?}", flag.is_true);

    let mut reference_mut = flag.is_true.borrow_mut();
    *reference_mut = false; // Dereference first to access inside
    println!("{}", reference_mut);
    println!("{:?}", flag.is_true);


    
}
