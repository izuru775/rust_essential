enum Location {
    Unkown,
    Anonymous,
    Known(f64,f64)
}
impl Location{
    fn display(&self){
        match *self{
            Location::Anonymous=>println!("location is anonymous"),
            Location::Unkown =>println!("location is known"),
            Location::Known(lat,lon) => println!("latitude is {} and longitude is {}",lat,lon)
        }
    }
}
fn main() {
    let address = Location::Anonymous;
    address.display();
    let address = Location::Unkown;
    address.display();
    let address = Location::Known(28.608295,-80.604177);
    address.display();

}
