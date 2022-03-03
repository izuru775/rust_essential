fn main() {
    let countdown = [5,4,3,2,1];
    let number = countdown.get(2); // get() method returns a "option enum" holding a refernce to a value at the specified enum
    let number = match number{
        Some(number)=>number+1,
        None => 0
    };
    println!("Number is {:?}",number);
}
