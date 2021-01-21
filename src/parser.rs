use crate::{Lexer, Program, Token};

#[derive(Debug)]
pub struct Parser {
    lexer: Lexer,
    cur_token: Option<Token>,
    peek_token: Option<Token>,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        let mut parser = Parser {
            lexer,
            cur_token: None,
            peek_token: None,
        };
        parser.next_token();
        parser.next_token();
        parser
    }

    fn next_token(&mut self) {
        if let Some(peek_token) = self.peek_token.clone() {
            self.cur_token = Some(peek_token);
            self.peek_token = Some(self.lexer.next_token());
        } else {
            self.peek_token = Some(self.lexer.next_token());
        }
    }

    pub fn parse_program(&self) -> Program {
        Program {
            statements: Vec::new(),
        }
    }
}
