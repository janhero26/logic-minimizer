mod ast;
mod token;
mod parser;

use token::tokenize;
use parser::Parser;

fn main() {
    let tokens = tokenize("(A)");
    let mut parser = Parser::new(tokens);
    let formula = parser.parse_formula();
    println!("{}", formula);
}