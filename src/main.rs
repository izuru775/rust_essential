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
use std::mem;

struct Shuttle{
    name:String,
    crew_size:u8,
    propellant:f64
}

fn main() {
    let vehicle =Shuttle{
        name:String::from("Atlantis"),
        crew_size:7,
        propellant:835958.0
    };

    println!("vehicle size on stack : {} bytes",mem:: size_of_val(&vehicle));

    let boxed_vehicle:Box<Shuttle> = Box::new(vehicle);
    println!("boxed_vehicle size on stack : {} bytes",mem:: size_of_val(&boxed_vehicle));
    println!("boxed_vehicle size on heap : {} bytes",mem:: size_of_val(&*boxed_vehicle));

    let unboxed_vehicle:Shuttle = *boxed_vehicle;
    println!("unboxed_vehicle size on stack : {} bytes",mem:: size_of_val(&unboxed_vehicle));

}
