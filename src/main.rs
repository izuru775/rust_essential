fn best_fuel<'a,'b>(x: &'a str, y: &'b str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        x
    }
}

fn main() {
    let result;
    let propellent1 = String::from("RP-1");
    {
        let propellent2 = String::from("LNG");
        result = best_fuel(&propellent1, &propellent2);
    }
    println!("result is {}", result);
}
