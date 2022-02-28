// Variables in rust are immutable by default

/*
    Local variables : snake_case
    Modules : snake_case
    Types : UpperCamelCase
    Traits : UpperCamelCase
    Enum variants : UpperCamelCase
    functions : snake_case
    Methods : snake_case
*/

struct Rectangle {
    widht: f64,
    height: f64,
}
impl Rectangle {
    fn get_area(&self) -> f64 {
        self.widht * self.height
    }

    fn scale(&mut self, scale: f64) {
        self.widht = self.widht * scale;
        self.height = self.height * scale;
    }
    fn new(widht: &f64, height: &f64) -> Rectangle {
        Rectangle {
            widht: f64,
            height: f64,
        }
    }
}

fn main() {}
