use std::option;



#[derive(Debug)]
struct item{
    name: String,
}

#[derive(Debug)]
struct Range{
    start: u32,
    end: u32,
}

impl Iterator for Range{
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item>{
        if self.start >= self.end{
            return None;
        }
        let result = Some(self.start);
        self.start += 1;
        result
    }
}
// struct City{
//     city: String,
//     population: u64,
// }

// fn sort_pop(city: &mut Vec<City>){
//     city.sort_by_key(pop_helper);
// }

// fn pop_helper(pop: &City) -> u64{
//     pop.population
// }

// fn sort_pop_closure(pop: &mut Vec<City>){
//     pop.sort_by_key(|p| p.population)
// }

fn check_inventory(items: Vec<item>, product: String) -> Vec<item>{
    items.into_iter().filter(|i| i.name == product).collect()
}
fn main() {
    // let a = City{city:String::from("A"), population: 100};
    // let b = City{city:String::from("B"), population: 55};
    // let c = City{city:String::from("C"), population: 200};
    // let d = City{city:String::from("D"), population: 29};
    // let e = City{city:String::from("E"), population: 90};

    // let mut vec: Vec<City> = Vec::new();
    // vec.push(a);
    // vec.push(b);
    // vec.push(c);
    // vec.push(d);
    // vec.push(e);

    // println!("{:?}", vec);
    // // sort_pop(&mut vec);
    // sort_pop_closure(&mut vec);
    // println!("{:?}", vec);

    // let add  = |x: i32| -> i32 {x + 1};
    // let add_v2 = |x| x + 1;
    // // add_v2(1);
    // // println!("{:}", add_v2); // Closures in Rust do not automatically implement the Debug trait, which means you cannot directly pass them as printable objects to println!.
    // println!("{:}", add_v2(1));

    // let example = |x| x;
    // let string = example(String::from("Jacob"));
    // let num = example(1); // The compiler infers the type of x when the closure is first called. For example, if example(String::from("Jacob")) is called, x is inferred as string.

    
    //Fn, FnMut, FnOnce
    //Fn including all fn functions

    // || drop(v) FnOnce
    // |args| v.contains(args) Fn
    // |args| v.push(args) FnMut

    // let y = 5;
    // let add_y = |x| x + y;
    // let copy = add_y;
    // println!("{}", add_y(copy(10)));


    /*
        This line tries to copy the closure add_y to copy, 
        but since add_y has already captured a mutable reference to y, 
        the copy causes potential ownership and borrowing conflicts.
     */
    // let mut y = 5;
    // let mut add_y = |x| {y += x; y};
    // let mut copy = add_y;
    // println!("{}", add_y(copy(10)));


    /*
        into_iter consumes the collection itself and takes ownership of each element within the collection.
     */
    // let vec2 = vec![1,2,3];
    // let mut iter = (&vec2).into_iter();
    // while let Some(v) = iter.next(){
    //     println!("{}", v);
    // }

    let mut vec: Vec<item> = Vec::new();
    vec.push(item { 
        name: String::from("coat") 
    });
    vec.push(item { 
        name: String::from("shirt") 
    });
    vec.push(item { 
        name: String::from("shorts") 
    });
    vec.push(item { 
        name: String::from("shoes") 
    });

    let checked = check_inventory(vec, String::from("shirt"));
    println!("{:?}", checked);


    let mut range = Range{start: 0, end: 10};
    // for r in range{
    //     println!("{}", r);
    // }
    
    let vec2: Vec<u32> = range.filter(|x| x % 2 == 0).collect();
    println!("{:?}", vec2);


}
