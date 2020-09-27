#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Token {
    Illegal,       // unknown token
    Eof,           // end of file
    Ident(String), // identifier
    Int(usize),    // integer literal
    Assign,        // =
    Plus,          // +
    Minus,         // -
    Bang,          // !
    Asterisk,      // *
    Slash,         // /
    Lt,            // <
    Gt,            // >
    Comma,         // ,
    Semicolon,     // ;
    Lparen,        // (
    Rparen,        // )
    Lbrace,        // {
    Rbrace,        // }
    Function,      // fn
    Let,           // let
}
