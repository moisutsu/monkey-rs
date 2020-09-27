use crate::Token;

static SINGLE_LETTER_TOKENS: [Token; 16] = [
    Token::Illegal,
    Token::Eof,
    Token::Assign,
    Token::Plus,
    Token::Minus,
    Token::Bang,
    Token::Asterisk,
    Token::Slash,
    Token::Lt,
    Token::Gt,
    Token::Comma,
    Token::Semicolon,
    Token::Lparen,
    Token::Rparen,
    Token::Lbrace,
    Token::Rbrace,
];

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
    fn read_char(&mut self) {
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
            Some('=') => {
                if self.peek_char() == '=' {
                    self.read_char();
                    self.read_char();
                    Token::Eq
                } else {
                    Token::Assign
                }
            }
            Some('+') => Token::Plus,
            Some('-') => Token::Minus,
            Some('!') => {
                if self.peek_char() == '=' {
                    self.read_char();
                    self.read_char();
                    Token::NotEq
                } else {
                    Token::Bang
                }
            }
            Some('*') => Token::Asterisk,
            Some('/') => Token::Slash,
            Some('<') => Token::Lt,
            Some('>') => Token::Gt,
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
        // If `token` is not a single letter token, `read_char` is already called
        if SINGLE_LETTER_TOKENS.contains(&token) {
            self.read_char();
        }
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
    fn peek_char(&self) -> char {
        if self.read_position >= self.input.len() {
            '\u{0}'
        } else {
            self.input.chars().nth(self.read_position).unwrap()
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
        "true" => Token::True,
        "false" => Token::False,
        "if" => Token::If,
        "else" => Token::Else,
        "return" => Token::Return,
        _ => Token::Ident(ident),
    }
}
