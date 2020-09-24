#[derive(Debug, Default)]
pub struct Lexer {
    input: String,
    position: i32,
    read_position: i32,
    ch: char,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        Lexer {
            input,
            ..Lexer::default()
        }
    }
}
