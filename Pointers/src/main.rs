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
}
