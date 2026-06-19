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
            '-' => {
                chars.next();
                if chars.peek() == Some(&'>') {
                    chars.next();
                    tokens.push(Token::Implies);
                }
            }
            '<' => {
                chars.next();
                if chars.peek() == Some(&'-') {
                    chars.next();
                    if chars.peek() == Some(&'>') {
                        chars.next();
                        tokens.push(Token::Iff);
                    }
                }
            }
            c if c.is_alphabetic() => {
                let mut name = String::new();
                while let Some(&c) = chars.peek() {
                    if c.is_alphanumeric() {
                        name.push(c);
                        chars.next();
                    } else {
                        break;
                    }
                }
                tokens.push(Token::Var(name));
            }
            _ => {
                chars.next();
            }
        }
    }

    tokens
}

#[cfg(test)]
mod tests {
    use super::*;

    fn var(name: &str) -> Token {
        Token::Var(name.to_string())
    }

    #[test]
    fn einfache_und_verknuepfung() {
        assert_eq!(tokenize("A & B"), vec![var("A"), Token::And, var("B")]);
    }

    #[test]
    fn klammern_und_oder() {
        assert_eq!(
            tokenize("A & (B | C)"),
            vec![var("A"), Token::And, Token::LParen, var("B"), Token::Or, var("C"), Token::RParen]
        );
    }

    #[test]
    fn mehrzeichige_operatoren() {
        assert_eq!(
            tokenize("A -> B <-> !C"),
            vec![var("A"), Token::Implies, var("B"), Token::Iff, Token::Not, var("C")]
        );
    }

    #[test]
    fn mehrbuchstabige_variable() {
        assert_eq!(tokenize("Foo & Bar"), vec![var("Foo"), Token::And, var("Bar")]);
    }
}