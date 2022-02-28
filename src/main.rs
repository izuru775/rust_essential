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



struct Shuttle{
    name:String,
    crew_size:u8,
    propellant:f64
}

impl Shuttle{
    fn get_name(&self)->&str{
        &self.name
    }

    fn add_fuel(&mut self,gallons:f64){
        self.propellant+=gallons;
    }
}

fn main() {

    let mut vehicle = Shuttle{
        name:String::from("Endeavour"),
        crew_size:8,
        propellant:0.0
    };

    let hehicle_name = vehicle.get_name();
    println!("vehicle name is {}",hehicle_name);

    println!("propellent is {}",vehicle.propellant);
    vehicle.add_fuel(1000.0);
    println!("propellent is {}",vehicle.propellant);

}
