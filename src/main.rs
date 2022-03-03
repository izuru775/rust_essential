fn best_fuel<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
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
