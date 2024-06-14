/*
 *
 * You can run Rust testing with mul_threads
 * ``cargo test -- --test-threads={numbers of threads you want}``
 * 
 * You can just run single test case
 * ``cargo test {function name}``
 * 
 * You can run subset name of test case
 * ``cargo test {sebset name}`` => For example, ``cargo test it`` will run function "it_fails" and "it_works"
 * 
 */



#[cfg(test)]
mod tests{
    use super::*; // Mention it if you need to use function out of scope.

    #[test]
    fn it_works(){
        let result = 2 + 2;
        assert_eq!(result, 4); // assert equal
        assert_ne!(result, 5); // assert not equal
    }


    #[test]
    // #[ignore] // Menthion it if you do not want to run it.
    #[should_panic] // Mention it if the result should panic
    fn it_fails(){
        panic!("Test Fail!");
    }

    #[test]
    fn call_simple_add(){
        assert!(simple_add());
    }
}

fn simple_add() -> bool {
    if 2 + 2 == 4{
        true
    }
    else{
        false
    }
}