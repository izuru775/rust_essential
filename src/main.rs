fn main() {
    let countdown = [5,4,3,2,1];
    let number = countdown.get(5); // get() method returns a "option enum" holding a refernce to a value at the specified enum
    let number = number.unwrap_or(&0) +1;
    println!("Number is {:?}",number);
}
