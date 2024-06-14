use std::fs::File;
use std::io::ErrorKind;
use std::fs::rename;
use std::io::Error;
fn main() {
    /*
     * Panic
     */

    // panic!("Panic Here");

    // // panic index out of bounds
    // let vec = vec![1];
    // vec[10];

    /*
     *  Result
     */

    // let file = File::open("error.txt");

    /*
     *  Catching Error
     */

    //  let file = File::open("error.txt");
    //  let file = match file{
    //     Ok(file) => file,
    //     // Err(error) => panic!("Error: {:?}", error),
    //     Err(error) => match error.kind(){
    //         ErrorKind::NotFound => match File::create("error.txt"){
    //             Ok(file_create) => file_create,
    //             Err(err) => panic!("Cannot create a file."),
    //         }
    //         _ => panic!("There are some others error kind."),
    //     }
    //  };

    // // Using unwrap get the error
    // let file = File::open("error.txt").unwrap();

    // // Using expect get the error
    // let file = File::open("error.txt").expect("Error opening the files");

    /*
     *  Error Propagation
     */

    // let test = open_file();
    // test.unwrap();

    rename_file().unwrap();
}

// enum Result<T, E>{ // It is Result looks like.
//     Ok(T),
//     Err(E),
// }

fn open_file() -> Result<File, Error> {
    let file  = File::open("error.txt")?;
    Ok(file)
}

fn rename_file() -> Result<(), Error>{
    let file = rename("error.txt", "rename.txt")?;
    Ok(file)
}