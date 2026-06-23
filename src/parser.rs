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
        self.parse_iff()
    }

    fn parse_not(&mut self) -> Formula {
        if self.peek() == Some(&Token::Not) {
            self.advance(); // ! verbrauchen
            let operand = self.parse_not();
            Formula::Not(Box::new(operand))
        } else {
            self.parse_atom()
        }
    }

    fn parse_and(&mut self) -> Formula {
        let mut left = self.parse_not();

        while self.peek() == Some(&Token::And) {
            self.advance(); // & verbrauchen
            let right = self.parse_not();
            left = Formula::And(Box::new(left), Box::new(right));
        }

        left
    }

    fn parse_or(&mut self) -> Formula {
        let mut left = self.parse_and();

        while self.peek() == Some(&Token::Or) {
            self.advance();
            let right = self.parse_and();
            left = Formula::Or(Box::new(left), Box::new(right));
        }

        left
    }

    fn parse_implies(&mut self) -> Formula {
        let left = self.parse_or();

        if self.peek() == Some(&Token::Implies) {
            self.advance();
            let right = self.parse_implies();
            Formula::Implies(Box::new(left), Box::new(right))
        } else {
            left
        }
    }

    fn parse_iff(&mut self) -> Formula {
        let left = self.parse_implies();

        if self.peek() == Some(&Token::Iff) {
            self.advance();
            let right = self.parse_iff();
            Formula::Iff(Box::new(left), Box::new(right))
        } else {
            left
        }
    }
}