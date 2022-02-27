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
    let mut speech = String::new();
    speech.push_str("We choose to go to the Moon in this decade\n");
    speech.push_str("and do other things,\n");
    speech.push_str("not because they are easy,\n");
    speech.push_str("but because they are hard.\n");

    fs::write("speech.txt",speech);
}
