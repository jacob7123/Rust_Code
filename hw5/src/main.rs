trait set_up_data{
    fn set_mpg(&mut self, new_mpg: f32);
    fn set_color(&mut self, new_color: String);
    fn set_top_speed(&mut self, new_top_speed: f32);
}

#[derive(Debug)]
struct Car {
    mpg: f32,
    color: String,
    top_speed: f32,
}

#[derive(Debug)]
struct Motorcycle {
    mpg: f32,
    color: String,
    top_speed: f32,
}

impl set_up_data for Car {
    fn set_mpg(&mut self, new_mpg: f32){
        self.mpg = new_mpg;
        println!("Car mpg reset ready!");
    }
    fn set_color(&mut self, new_color: String){
        self.color = new_color;
        println!("Car color reset ready!");
    }
    fn set_top_speed(&mut self, new_top_speed: f32){
        self.top_speed = new_top_speed;
        println!("Car top speed reset ready!");
    }
}

impl set_up_data for Motorcycle {
    fn set_mpg(&mut self, new_mpg: f32){
        self.mpg = new_mpg;
        println!("Motorcycle mpg reset ready!");
    }
    fn set_color(&mut self, new_color: String){
        self.color = new_color;
        println!("Motorcycle color reset ready!");
    }
    fn set_top_speed(&mut self, new_top_speed: f32){
        self.top_speed = new_top_speed;
        println!("Motorcycle top speed reset ready!");
    }
}

fn print<T: std::fmt::Debug>(value: T){
    println!("{:?}", value);
}

fn main() {
    let mut car        = Car{mpg: 30.0, color: String::from("Red"), top_speed: 180.0};
    let mut motorcycle = Motorcycle{mpg: 40.0, color: String::from("Green"), top_speed: 100.0};

    println!("{:?}", car);
    println!("{:?}", motorcycle);

    car.set_mpg(31.0);
    car.set_color(String::from("Black"));
    car.set_top_speed(200.0);

    motorcycle.set_mpg(50.0);
    motorcycle.set_color(String::from("White"));
    motorcycle.set_top_speed(120.0);

    println!("{:?}", car);
    println!("{:?}", motorcycle);


    print(vec![1,2,3,4,5,6,7,8,9,9]);
    print(car);
    print(motorcycle);
}
