use crate::ast::Formula;

pub fn simplify(formula: Formula) -> Formula {
    match formula {
        Formula::Var(name) => Formula::Var(name),

        Formula::Not(inner) => {
            let inner = simplify(*inner);
            match inner {
                Formula::Not(deeper) => *deeper,
                other => Formula::Not(Box::new(other)),
            }
        }

        Formula::And(left, right) => {
            let left = simplify(*left);
            let right = simplify(*right);
            Formula::And(Box::new(left), Box::new(right))
        }

        Formula::Or(left, right) => {
            let left = simplify(*left);
            let right = simplify(*right);
            Formula::Or(Box::new(left), Box::new(right))
        }

        Formula::Implies(left, right) => {
            let left = simplify(*left);
            let right = simplify(*right);
            Formula::Implies(Box::new(left), Box::new(right))
        }

        Formula::Iff(left, right) => {
            let left = simplify(*left);
            let right = simplify(*right);
            Formula::Iff(Box::new(left), Box::new(right))
        }
    }
}