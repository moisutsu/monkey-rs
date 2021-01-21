use std::process::exit;

use monkey::{Identifier, Lexer, Parser, Statement};

#[test]
fn test_let_statements() {
    let input = "
    let x = 5;
    let y = 10;
    let foobar = 838383;
    "
    .to_string();
    let lexer = Lexer::new(input);
    let parser = Parser::new(lexer);

    let program = parser.parse_program();

    if program.statements.len() == 0 {
        panic!("Parser.parse_program() returned nothing.");
    }

    let expected_idents = vec!["x", "y", "foobar"];
    for (i, &expected_ident) in expected_idents.iter().enumerate() {
        let statement = program.statements[i].clone();
        assert!(test_let_statement(statement, expected_ident));
    }
}

fn test_let_statement(statement: Statement, expected_ident: &str) -> bool {
    match &statement.token_literal()[..] {
        "let" => (),
        literal => {
            eprintln!("Token literal is not 'let'. Got={}", literal);
            return false;
        }
    }
    match statement {
        Statement::LetStatement(let_statement) => {
            let Identifier(name) = let_statement.name;
            if expected_ident != &name {
                eprintln!("Identifier name is not '{}'. Got {}.", expected_ident, name);
                return false;
            }
        }
        _ => {
            eprintln!("statement is not Statement::LetStatement");
            return false;
        }
    }
    true
}
