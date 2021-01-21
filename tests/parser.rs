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
        test_let_statement(statement, expected_ident);
    }
}

fn test_let_statement(statement: Statement, expected_ident: &str) {
    match &statement.token_literal()[..] {
        "let" => (),
        literal => {
            panic!("Token literal is not 'let'. Got={}", literal);
        }
    }
    match statement {
        Statement::LetStatement(let_statement) => {
            let Identifier(name) = let_statement.name;
            if expected_ident != &name {
                panic!("Identifier name is not '{}'. Got {}.", expected_ident, name);
            }
        }
        _ => {
            panic!("statement is not Statement::LetStatement");
        }
    }
}
