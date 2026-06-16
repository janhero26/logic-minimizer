mod ast;

use ast::Formula;

fn main() {
    // A ∧ (A ∨ B)
    let formula = Formula::And(
        Box::new(Formula::Var("A".to_string())),
        Box::new(Formula::Or(
            Box::new(Formula::Var("A".to_string())),
            Box::new(Formula::Var("B".to_string())),
        )),
    );

    println!("{}", formula);
}