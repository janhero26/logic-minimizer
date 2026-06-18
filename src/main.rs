mod ast;
mod token;

use token::tokenize;

fn main() {
    let input = "A & (B | C)";
    let tokens = tokenize(input);
    println!("{:?}", tokens);
}