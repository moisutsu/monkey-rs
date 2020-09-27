use monkey::{Lexer, Token::*};

#[test]
fn test_next_token() {
    let input = "let five = 5;
    let ten = 10;

    let add = fn(x, y) {
        x + y;
    };
    let result = add(five, ten);"
        .to_string();
    let answers = vec![
        Let,
        Ident("five".to_string()),
        Assign,
        Int(5),
        Semicolon,
        Let,
        Ident("ten".to_string()),
        Assign,
        Int(10),
        Semicolon,
        Let,
        Ident("add".to_string()),
        Assign,
        Function,
        Lparen,
        Ident("x".to_string()),
        Comma,
        Ident("y".to_string()),
        Rparen,
        Lbrace,
        Ident("x".to_string()),
        Plus,
        Ident("y".to_string()),
        Semicolon,
        Rbrace,
        Semicolon,
        Let,
        Ident("result".to_string()),
        Assign,
        Ident("add".to_string()),
        Lparen,
        Ident("five".to_string()),
        Comma,
        Ident("ten".to_string()),
        Rparen,
        Semicolon,
        Eof,
    ];
    let mut lexer = Lexer::new(input);
    for i in 0..answers.len() {
        assert_eq!(lexer.next_token(), answers[i]);
    }
}