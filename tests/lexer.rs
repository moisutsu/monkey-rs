use monkey::{Lexer, Token::*};

#[test]
fn test_next_token() {
    let input = "let five = 5;
    let ten = 10;

    let add = fn(x, y) {
        x + y;
    };
    let result = add(five, ten);
    !-/*5;
    5 < 10 > 5;
    if (5 < 10) {
        return true;
    } else {
        return false;
    }
    "
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
        Bang,
        Minus,
        Slash,
        Asterisk,
        Int(5),
        Semicolon,
        Int(5),
        Lt,
        Int(10),
        Gt,
        Int(5),
        Semicolon,
        If,
        Lparen,
        Int(5),
        Lt,
        Int(10),
        Rparen,
        Lbrace,
        Return,
        True,
        Semicolon,
        Rbrace,
        Else,
        Lbrace,
        Return,
        False,
        Semicolon,
        Rbrace,
        Eof,
    ];
    let mut lexer = Lexer::new(input);
    for i in 0..answers.len() {
        assert_eq!(lexer.next_token(), answers[i]);
    }
}
