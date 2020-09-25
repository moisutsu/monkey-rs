#[derive(Debug, Default)]
pub struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    ch: Option<char>,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        Lexer {
            input,
            ..Lexer::default()
        }
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
}
