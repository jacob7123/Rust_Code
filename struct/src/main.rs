// struct User{
//     active: bool,
//     username: String,
//     sign_in_account: u32,
// }

// struct Square{
//     width: u32,
//     height: u32,
// }
// impl Square{
//     fn area(&self) -> u32{
//         self.width * self.height
//     }
//     fn get_width(&self) -> u32{
//         self.width
//     }
//     fn get_height(&self) -> u32{
//         self.height
//     }
//     fn set_width(&mut self, new_width: u32){
//         self.width = new_width;
//     }
// }

struct MyString<'a>{
    text: &'a str,
}

fn main() {
    /*
        Introduce Struct
     */
    // let user1 = User{active: true, username: String::from("Jacob"), sign_in_account: 0};
    // println!("{}", user1.username);

    // let user2 = build_username(String::from("Huang"));
    // println!("{}", user2.username);

    /*
        Method
     */
    // let mut sq = Square{width: 5, height: 6};
    // println!("{}", sq.area());
    // println!("{}", sq.get_width());
    // println!("{}", sq.get_height());

    // sq.set_width(100);
    // println!("{}", sq.area());
    // println!("{}", sq.get_width());
    // println!("{}", sq.get_height());

    /*
        LifeTime
     */
    // let r;
    // {
    //     let x = 5;
    //     r     = &x;
    // }// x is dropped
    // println!("{}", r);
    // &i32
    // &'a i32
    // &'a mut i32

    /*
        LifeTime in Struct
     */
    let str1 = String::from("It is my string.");
    let x = MyString{text: str1.as_str()};
    let s: &'static str = "I have a static lifetime";
}

// fn lifetime_example<'a, 'b>(x: &'a str, y: &'b str) -> &'a str{
//     x
// }// 'a for one parameter, and 'b for second parameter

// fn build_username(username: String) -> User{
//     User{
//         username,
//         active: true,
//         sign_in_account: 1,
//     }
// }