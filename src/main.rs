#[derive(Debug)]
enum Shapes{
    Circle(f64), // radius
    Rectangle(f64,f64), // width, height
    Triangle(f64,f64,f64) // sides a,b,c
}

impl Shapes{
    fn get_perimeter(&self)->f64{
        match *self{
            Shapes::Circle(r)=> r * 2.0 * std::f64::consts::PI,
            Shapes::Rectangle(w,h) =>(2.0 * w) + (2.0 * h),
            Shapes::Triangle(a,b,c) => a+b+c
        }
    }
}
fn main() {
    let my_shape = Shapes::Rectangle(1.2,3.4);
    println!("my_shape is {:?}",my_shape);

    let perimeter = my_shape.get_perimeter();
    println!("perimeter is {}",perimeter);
}
