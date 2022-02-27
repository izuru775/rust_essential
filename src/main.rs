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

use std::env;

fn main() {
    if env::args().len()<= 2{
        println!("Programe requires at least 2 arguments.");
        return;
    }
    for (index, argument) in env::args().enumerate() {
        println!("Argument {} is {}", index, argument);
    }

    let arg2 = env::args().nth(2).unwrap();
    println!("arg2 is {}", arg2);
}
