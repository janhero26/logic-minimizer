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

pub fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(&c) = chars.peek() {
        match c {
            ' ' | '\t' | '\n' => {
                chars.next();
            }
            '&' => {
                tokens.push(Token::And);
                chars.next();
            }
            '|' => {
                tokens.push(Token::Or);
                chars.next();
            }
            '!' => {
                tokens.push(Token::Not);
                chars.next();
            }
            '(' => {
                tokens.push(Token::LParen);
                chars.next();
            }
            ')' => {
                tokens.push(Token::RParen);
                chars.next();
            }
            _ => {
                chars.next();
            }
        }
    }

    tokens
}