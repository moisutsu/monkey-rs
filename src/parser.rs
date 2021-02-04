use crate::{Expression, Identifier, LetStatement, Lexer, Program, Statement, Token};

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

    pub fn parse_program(&mut self) -> Program {
        let mut program = Program {
            statements: Vec::new(),
        };
        while self.cur_token != Some(Token::Eof) {
            let statement = self.parse_statement();
            if let Some(statement) = statement {
                program.statements.push(statement);
            }
            self.next_token();
        }
        program
    }

    pub fn parse_statement(&mut self) -> Option<Statement> {
        match self.cur_token.clone()? {
            Token::Let => Some(self.parse_let_statement()?),
            _ => None,
        }
    }

    fn parse_let_statement(&mut self) -> Option<Statement> {
        if self.cur_token != Some(Token::Let) {
            return None;
        }
        let ident;
        match self.peek_token.clone() {
            Some(Token::Ident(name)) => {
                self.next_token();
                ident = Identifier(name.clone());
            }
            _ => {
                return None;
            }
        };
        if !self.expect_peek(Token::Assign) {
            return None;
        }
        while !self.cur_token_is(Token::Semicolon) {
            self.next_token();
        }
        Some(Statement::LetStatement(LetStatement {
            name: ident.clone(),
            value: Expression::Identifier(ident),
        }))
    }

    fn cur_token_is(&self, token: Token) -> bool {
        self.cur_token == Some(token)
    }

    fn peek_token_is(&self, token: Token) -> bool {
        self.peek_token == Some(token)
    }

    fn expect_peek(&mut self, token: Token) -> bool {
        if self.peek_token_is(token) {
            self.next_token();
            true
        } else {
            false
        }
    }
}
