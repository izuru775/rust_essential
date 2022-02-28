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
#[derive(Debug)]
#[derive(Clone)]


struct Shuttle{
    name:String,
    crew_size:u8,
    propellant:f64
}

fn main() {

    let mut vehicle = Shuttle{
        name:String::from("Endeavour"),
        crew_size:8,
        propellant:835958.0
    };

    let mut vehicle2 = Shuttle{
        ..vehicle.clone()
    };
    
    vehicle.crew_size = 6;
    

    println!("vehicle is {:?}",vehicle);
    println!("vehicle2 is {:?}",vehicle2);

}
