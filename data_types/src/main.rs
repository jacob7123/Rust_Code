fn main() {
    // let x: i8 = 10; // 8 bits int
    // println!("{}", x);

    // let y: u8 = 10; // unsign 8 bits int 
    // println!("{}", y);

    // let decimal = 02_55;
    // let hex = 0xff;
    // let octal = 0o377;
    // let binary = 0b1111_1111;

    // println!("{}", decimal);
    // println!("{}",hex);
    // println!("{}", octal);
    // println!("{}", binary);

    // let byte = b'A';
    // println!("{}", byte);


    // let x = 2.0; // float 64 bits. It is default because of modern CPUs 
    // let y: f32 = 2.0; // float 32 bits

    // let t = true; // Bool
    // let f: bool = false;

    // let c = 'c'; // char
    // println!("{}", c);

    // let a = 10;
    // let b = 4;

    // let remainder = a % b;
    // println!("{}", remainder);

    /*
        Tuple
     */

    // let tup = (500, "hi", true);
    // println!("{}", tup.0);
    // println!("{}", tup.1);
    // println!("{}", tup.2);

    // let (x, y, z) = tup;
    // println!("{}", x);
    // println!("{}", y);
    // println!("{}", z);

    /*
        Array
     */
    // let array = [1, 2, 3];
    // println!("{}", array[0]);
    // println!("{}", array[1]);
    // println!("{}", array[2]);

    // let array2: [i32; 3] = [4, 5, 6]; 
    // println!("{}", array2[0]);
    // println!("{}", array2[1]);
    // println!("{}", array2[2]);

    // array2[0] = 10; // You need to use 'mut'.  'let mut array2'

    /*
        Vector
     */
    // let mut nums = vec![1, 2, 3];
    // nums.push(4); // Push to last
    // println!("{:?}", nums);
    // nums.pop(); // Remove last one
    // println!("{:?}", nums);

    // let mut vec = Vec::new(); //vec!
    // vec.push("Test");
    // vec.push("String");
    // println!("{:?}", vec);

    // vec.reverse();
    // println!("{:?}", vec);
    // println!("{}", vec.capacity());

    // let mut vect = Vec::<i32>::with_capacity(2);
    // println!("{}", vect.capacity());

    // let v: Vec<i32> = (0..5).collect();
    // println!("{:?}", v);

    /*
        Slices
     */
    // let v: Vec<i32> = (0..5).collect();
    // println!("{:?}", v);
    // let sv1: &[i32] = &v;
    // println!("{:?}", sv1);
    // let sv2: &[i32] = &v[2..4];
    // println!("{:?}", sv2);

    /*
        String and &str
     */
    // let name = String::from("Jacob");
    // let course = "Rust".to_string();
    // let new_name = name.replace("Jacob", "Chingyen");
    // println!("{}", name);
    // println!("{}", course);
    // println!("{}", new_name);

    // // &str = "string slice" or "stir"
    // let str1 = "Hello"; //$str
    // let str2 = str1.to_string();
    // let str3 = &str2;
    // println!("{}", str1);
    // println!("{}", str2);
    // println!("{}", str3);

    // //compare string using "==" or "!="
    // println!("{}", "ONE".to_lowercase() == "one");

    /*
        String literals
     */
    let rust = "\x52\x75\x73\x74";
    println!("{}", rust);
}
