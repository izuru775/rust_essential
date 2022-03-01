struct Satellite{
    name:String,
    velocity:f64 // miles per second
}

struct SpaceStation{
    name:String,
    crew_size:u8,
    altitude:u32 // miles
}
// Trait structure
trait Description{
    fn describe(&self)->String;
}

// Implementing trait to Satellite struct
impl Description for Satellite{
    fn describe(&self)->String{
        format!("The {} is flying at {} miles per second",self.name,self.velocity)
    }
}

// Implementing trait to SpaceStation struct
impl Description for SpaceStation{
    fn describe(&self)->String{
        format!("The {} is flying {} miles hight with {} crew members on board!",self.name,self.altitude,self.crew_size)
    }
}


fn main() {
    let hubble = Satellite{
        name:String::from("Hubble Telescope"),
        velocity:4.72
    };

    let iss = SpaceStation{
        name:String::from("International Space Station"),
        crew_size:6,
        altitude:254
    };

    println!("hubble is : {}",hubble.describe());
    println!("iss is : {}",iss.describe());
    
}
