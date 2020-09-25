#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Token {
    Illegal,       // unknown token
    Eof,           // end of file
    Ident(String), // identifier
    Int(i32),      // integer literal
    Assign,        // =
    Plus,          // +
    Comma,         // ,
    Semicolon,     // ;
    Lparen,        // (
    Rparen,        // )
    Lbrace,        // {
    Rbrace,        // }
    Function,      // fn
    Let,           // let
}
