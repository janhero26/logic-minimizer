mod ast;
mod token;
mod parser;
mod simplify;

use token::tokenize;
use parser::Parser;
use simplify::simplify;

fn main() {
    let tokens = tokenize("!!A");
    let mut parser = Parser::new(tokens);
    let formula = parser.parse_formula();
    let simplified = simplify(formula);
    println!("{}", simplified);
}