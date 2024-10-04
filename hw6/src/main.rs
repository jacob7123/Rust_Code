fn main() {
    /*
        Create a vector with the values 1, 3, 5, 7, and 9. 
        Then use an iterator and a closure to multiply all of the values by 10 and store the result in another vector. 
        Print out the vector to confirm your results.
     */
    let vec: Vec<u32> = vec![1, 3, 5, 7, 9];
    let ans: Vec<u32> = vec.iter().map(|x| x * 10).collect();

    println!("{:?}", ans);
}
