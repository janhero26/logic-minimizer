use crate::ast::Formula;
use crate::token::Token;

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, pos: 0 }
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    fn advance(&mut self) -> Option<Token> {
        let token = self.tokens.get(self.pos).cloned();
        if token.is_some() {
            self.pos += 1;
        }
        token
    }

    fn parse_atom(&mut self) -> Formula {
        match self.advance() {
            Some(Token::Var(name)) => Formula::Var(name),
            Some(Token::LParen) => {
                let inner = self.parse_formula();
                self.advance(); // schließende ) verbrauchen
                inner
            }
            other => panic!("Unerwartetes Token: {:?}", other),
        }
    }

    pub fn parse_formula(&mut self) -> Formula {
        self.parse_atom()
    }
}