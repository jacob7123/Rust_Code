struct Car{
    mpg: i32,
    color: String,
    top_speed: i32,
}

impl Car{
    fn set_mpg(&mut self, new_mpg: i32){
        self.mpg = new_mpg;
    }
    fn set_color(&mut self, new_color: String){
        self.color = new_color;
    }
    fn set_top_speed(&mut self, new_top_speed: i32){
        self.top_speed = new_top_speed;
    }
}



fn main() {
    let mut car = Car{mpg: 30, color: String::from("Red"), top_speed: 180};
    println!("This car mpg is {}", car.mpg);
    println!("This car color is {}", car.color);
    println!("This car top speed is {}", car.top_speed);

    car.set_mpg(40);
    car.set_color(String::from("Black"));
    car.set_top_speed(200);
    println!("");
    println!("After fixed");
    println!("This car mpg is {}", car.mpg);
    println!("This car color is {}", car.color);
    println!("This car top speed is {}", car.top_speed);
}
