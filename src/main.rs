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
use std::env;

fn main() {

    if env::args().len() <= 2 {
        eprintln!("Program Require two arguments: <file Path> <search name>");
        std::process::exit(1);
    }
    let  file_name = env::args().nth(1).unwrap();
    let  walker_name = env::args().nth(2).unwrap();

    for lines in fs::read_to_string(file_name).unwrap().lines(){
        if lines == walker_name{
            println!("{} did walk on the Moon!",walker_name);
            return;
        }
    }

    println!("{} did NOT walk on the Moon....YET!",walker_name);
       
}
