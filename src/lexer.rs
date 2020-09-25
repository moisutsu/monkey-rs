#[derive(Debug, Default)]
pub struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
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
