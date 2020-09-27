use std::io::{stdin, stdout, Write};

pub fn start() {
    let prompt = ">> ";
    let stdin = stdin();
    loop {
        print!("{}", prompt);
        stdout().flush().unwrap();
        let mut buf = String::new();
        if let Err(_) = stdin.read_line(&mut buf) {
            break;
        }
        let mut lexer = crate::Lexer::new(buf);
        loop {
            let token = lexer.next_token();
            if token == crate::Token::Eof {
                break;
            }
            println!("{:?}", token);
        }
    }
}
