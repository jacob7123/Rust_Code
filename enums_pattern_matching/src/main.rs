enum Pet{
    Dog,
    Cat, 
    Fish
}

impl Pet{
    fn what_am_I(&self) -> &'static str{
        match self{
            Pet::Dog => "I am a dog",
            Pet::Cat => "I am a cat",
            Pet::Fish => "I am a fish",
        }
    }
}

enum IpaddrKind{
    // V4,
    V4(String),
    V6,
}

struct IpAdrr{
    kind: IpaddrKind,
    address: String,
}

fn main() {
    /*
     *   Enums Basic
     */
    // let dog = Pet::Dog;
    // println!("{}", dog.what_am_I());

    // let home = IpAdrr{
    //     kind: IpaddrKind::V4,
    //     address: String::from("127.0.0.1"),
    // };
    // let home = IpaddrKind::V4(String::from("127.0.0.1"));

    // let loopack = IpAdrr{
    //     kind: IpaddrKind::V6,
    //     address: String::from("::1")
    // };

    /*
     *   Option
     *   enum Option<T> {
     *     None,
     *       Some(T),
     *   }
     */
    // let some_number = Some(5);
    // let some_string = Some("a string");
    // let nothing: Option<i32> = None;

    // let x: i32 = 5;
    // let y: Option<i32> = Some(5);

    // let sum = x + y; //It cannot work since x is int32 data type but y is Option<int32>.

    /*
     *  Matching
     */
    // let five = Some(5);
    // let six  = plus_one(five);
    // let none = plus_one(None);
    // println!("{:?}", none);

    // what_pet(&String::from("Dog"));
    // what_pet(&String::from("Human"));

    /*
        If Let
     */
    // let dog2 = Some(Pet::Dog);
    // let cat = Some(Pet::Cat);
    // if let Some(Pet::Dog) = cat{
    //     println!("The animal is dog");
    // }
    // else{
    //     println!("It is not a dog");
    // }

    // let mut stack = Vec::new();
    // stack.push(1);
    // stack.push(2);
    // stack.push(3);

    // while let Some(top) = stack.pop(){
    //     println!("{}", top);
    // }

    /*
        More Match
     */

    let x = 1;
    match x{
        1 | 2 => println!("one or two"),
        _     => println!("Not one or two"),
    }

    let y = 5;
    match y{
        1..=5 => println!("Match!"),
        _    => println!("Not match!"),
    } 

    let a = Some(10);
    let b = 5;
    match a{
        Some(10)          => println!("10!"),
        Some(a) if a == b => println!("Match!"),
        _                 => println!("Default!"),
    }

}

fn plus_one(x: Option<i32>) -> Option<i32>{
    match x{
        None => None,
        Some(i) => Some(i + 1)
    }
}

fn what_pet(intput: &str){
    match intput{
        "Dog"  => println!("I have a dog."),
        "Cat"  => println!("I have a cat."),
        "Fish" => println!("I have a fish."),
        _      => println!("I have no clue what pet you have."),
    }
}
