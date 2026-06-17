#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Var(String),  // variable
    And,          // &
    Or,           // |
    Not,          // !
    Implies,      // ->
    Iff,          // <->
    LParen,       // (
    RParen,       // )
}