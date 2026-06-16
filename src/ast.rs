#[derive(Debug, Clone, PartialEq)]
pub enum Formula {
    Var(String),
    Not(Box<Formula>),
    And(Box<Formula>, Box<Formula>),
    Or(Box<Formula>, Box<Formula>),
    Implies(Box<Formula>, Box<Formula>),
    Iff(Box<Formula>, Box<Formula>),
}

use std::fmt;

impl fmt::Display for Formula {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Formula::Var(name) => write!(f, "{}", name),
            Formula::Not(phi) => write!(f, "¬{}", phi),
            Formula::And(l, r) => write!(f, "({} ∧ {})", l, r),
            Formula::Or(l, r) => write!(f, "({} ∨ {})", l, r),
            Formula::Implies(l, r) => write!(f, "({} → {})", l, r),
            Formula::Iff(l, r) => write!(f, "({} ↔ {})", l, r),
        }
    }
}