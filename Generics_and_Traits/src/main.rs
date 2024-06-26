// struct Point<T, U>{
//     x: T,
//     y: U,
// }

use std::fmt::format;

trait Overview {
    fn ovweview(&self) -> String{
        String::from("This is Rust course.")
    }
}

struct Course{
    headline: String,
    author  : String,
}

impl Drop for Course{
    fn drop(&mut self){
        println!("Dropping {}", self.author);
    }
}

struct AnotherCourse{
    headline: String,
    author  : String,
}

struct DefaultCourse{
    headline: String,
    author  : String,
}

impl Overview for Course {
    fn ovweview(&self) -> String {
        format!("Course : {}, {}", self.author, self.headline)
    }
}

impl Overview for AnotherCourse {
    fn ovweview(&self) -> String {
        format!("Another Course: {}, {}", self.author, self.headline)
    }
}

impl Overview for DefaultCourse {}

use std::ops::Add;
#[derive(Debug)]

struct Point<T>{
    x: T,
    y: T,
}

impl<T> Add for Point<T>
    where
    T: Add<Output = T>{
        type Output = Self;
        fn add(self, rhs: Self) -> Self{
            Point{
                x: self.x + rhs.x,
                y: self.y + rhs.y,
            }
        }
    }


fn main() {

    /*
        Genetics
     */
    // let coord1 = Point{x: 5.0, y: 5.0};
    // let coord2 = Point{x: 'x', y: 5.0};
    
    /*
        Traits
     */
    let c1 = Course{headline:String::from("Headline!"), author: String::from("Jacob"), };
    let c2 = AnotherCourse{headline:String::from("Another Headline!"), author: String::from("Huang"),};
    let c3 = DefaultCourse{headline:String::from("Default Headline!"), author: String::from("Default"),};

    // println!("{}", c1.ovweview());
    // println!("{}", c2.ovweview());
    // println!("{}", c3.ovweview());

    /*
        Traits as Parameters
     */
    call_overview(&c1);
    call_overview(&c2);
    call_overview(&c3);


    /*
        Drop
     */
    // drop(c1);

    /*
        Clone
     */

    /*
        Operator Overloading
     */
    let crood1 = Point{x: 5.0, y: 5.0};
    let crood2 = Point{x: 1.0, y: 2.0};
    let sum = crood1 + crood2;
    println!("{:?}", sum);


}

fn call_overview(item: &impl Overview){ // These two different way can do the same thing.
// fn call_overview<T: Overview>(item: &T){
    println!("{}", item.ovweview());
}

//fn overview(item1: &impl Overview, item2: &impl Overview) //They can be different type of Overview
//fn overview<T: Overview>(item1: &T, item2: &T) //They should be same type of Overview
