enum Shape{
    triangle,
    square,
    pentagon,
    octagon,
}

impl Shape{
    fn corner(&self) -> &'static i32{
        match self{
            Shape::triangle => &3,
            Shape::square   => &4,
            Shape::pentagon => &5,
            Shape::octagon  => &8,
        }
    }
}


fn main() {
    let tri = Shape::triangle;
    let squ = Shape::square;
    let pen = Shape::pentagon;
    let oct = Shape::octagon;
    
    println!("{}", tri.corner());
    println!("{}", squ.corner());
    println!("{}", pen.corner());
    println!("{}", oct.corner());
}
