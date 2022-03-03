use std::fs;

fn main() {
    let content = fs::read_to_string("the_ultimate_question.txt").expect("Nobody knows it!");
    println!("content is: {:?}",content);
}
