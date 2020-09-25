use monkey::{Lexer, Token::*};

#[test]
fn test_next_token() {
    let input = "=+(){},;".to_string();
    let answers = vec![
        Assign, Plus, Lparen, Rparen, Lbrace, Rbrace, Comma, Semicolon,
    ];
    let mut lexer = Lexer::new(input);
    for i in 0..answers.len() {
        assert_eq!(lexer.next_token(), answers[i]);
    }
}
