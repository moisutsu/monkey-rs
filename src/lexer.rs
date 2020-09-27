use crate::Token;

#[derive(Debug, Default)]
pub struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    ch: Option<char>,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let mut lexer = Lexer {
            input,
            ..Lexer::default()
        };
        lexer.read_char();
        lexer
    }
    pub fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = None;
        } else {
            self.ch = self.input.chars().nth(self.read_position); // O(n)
        }
        self.position = self.read_position;
        self.read_position += 1;
    }
    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        let token = match self.ch {
            Some('=') => Token::Assign,
            Some('+') => Token::Plus,
            Some(',') => Token::Comma,
            Some(';') => Token::Semicolon,
            Some('(') => Token::Lparen,
            Some(')') => Token::Rparen,
            Some('{') => Token::Lbrace,
            Some('}') => Token::Rbrace,
            Some(ch) => {
                if is_letter(ch) {
                    let ident = self.read_identifier();
                    ident_to_token(ident)
                } else if ch.is_ascii_digit() {
                    let digit = self.read_number();
                    Token::Int(digit)
                } else {
                    Token::Illegal
                }
            }
            None => Token::Eof,
        };
        // Function, Let, Ident, Int already called `read_char`
        match token {
            Token::Function | Token::Let | Token::Ident(_) | Token::Int(_) => (),
            _ => self.read_char(),
        };
        token
    }
    fn read_identifier(&mut self) -> String {
        let position = self.position;
        // if self.ch == Some(ch) && is_letter(ch) `true` else `false`
        while self.ch.map_or(false, is_letter) {
            self.read_char();
        }
        self.input.chars().collect::<Vec<char>>()[position..self.position]
            .iter()
            .collect()
    }
    fn read_number(&mut self) -> usize {
        let position = self.position;
        while self.ch.map_or(false, |ch| ch.is_ascii_digit()) {
            self.read_char();
        }
        self.input.chars().collect::<Vec<char>>()[position..self.position]
            .iter()
            .collect::<String>()
            .parse()
            .unwrap()
    }
    fn skip_whitespace(&mut self) {
        while self.ch.map_or(false, |ch| ch.is_ascii_whitespace()) {
            self.read_char()
        }
    }
}

#[inline]
fn is_letter(ch: char) -> bool {
    // ch.is_ascii_alphabetic() || ch == '_'
    match ch {
        'a'..='z' | 'A'..='Z' | '_' => true,
        _ => false,
    }
}

#[inline]
fn ident_to_token(ident: String) -> Token {
    match &ident[..] {
        "fn" => Token::Function,
        "let" => Token::Let,
        _ => Token::Ident(ident),
    }
}
