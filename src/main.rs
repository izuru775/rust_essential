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

use std::fs;

fn main() {
    let contents = fs::read_to_string("planets.txt").unwrap();
    println!("Content is {}",contents);

    for line in contents.lines(){
        println!("Line is {}",line);
    }

    let contents = fs::read("planets.txt").unwrap();
    println!("Content is {:?}",contents);

}
