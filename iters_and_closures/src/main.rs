

#[derive(Debug)]
struct City{
    city: String,
    population: u64,
}

// fn sort_pop(city: &mut Vec<City>){
//     city.sort_by_key(pop_helper);
// }

// fn pop_helper(pop: &City) -> u64{
//     pop.population
// }

fn sort_pop_closure(pop: &mut Vec<City>){
    pop.sort_by_key(|p| p.population)
}

fn main() {
    let a = City{city:String::from("A"), population: 100};
    let b = City{city:String::from("B"), population: 55};
    let c = City{city:String::from("C"), population: 200};
    let d = City{city:String::from("D"), population: 29};
    let e = City{city:String::from("E"), population: 90};

    let mut vec: Vec<City> = Vec::new();
    vec.push(a);
    vec.push(b);
    vec.push(c);
    vec.push(d);
    vec.push(e);

    println!("{:?}", vec);
    // sort_pop(&mut vec);
    sort_pop_closure(&mut vec);
    println!("{:?}", vec);

}
