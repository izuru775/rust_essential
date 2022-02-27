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

use rand::prelude::*;
use std::io;

fn main() {
    let mut count = 0;
    let comp_number = thread_rng().gen_range(1..101);
    println!("Enter a number between 1-100 :");
    println!("You will loose if you guess wrong for 5 times...");
    loop {
        let mut buffer = String::new();
        io::stdin()
            .read_line(&mut buffer)
            .expect("Falied to read input line");

        let user_number: u32 = buffer.trim().parse().expect("Failed to parse the guess");

        if count ==5{
            println!("\nYou guess wrong 5 times,you loss!!!");
            println!("Would you like to play again?(y/n)");

            let mut respose_buffer = String::new();
            io::stdin()
            .read_line(&mut respose_buffer)
            .expect("Falied to read input line");           
            let user_response: String = respose_buffer.trim().parse().expect("Failed to parse the response");

            if user_response.to_uppercase() == "Y"{
                println!("You'll have five more chances!");
                count = 0;

            }else if user_response.to_uppercase() == "N"{
                break;
            }else{
                println!("{} Invalid response!",user_response);
                break;
            }
            
        }

        if user_number == comp_number {
            println!("\nCorrect!!! the secret number is {}", comp_number);
            break;
        } else if user_number < comp_number {
            println!("\n{} is too low, guess again!", user_number);
            count +=1;
        } else {
            println!("\n{} is too high, guess again!", user_number);
            count +=1;
        }
        
    }
}
