mod ast;
mod token;
mod parser;

use token::tokenize;

fn main() {
    let input = "A -> B <-> !C";
    let tokens = tokenize(input);
    println!("{:?}", tokens);
}